// A small web API over the order system, built with axum and tokio.
//   cargo run -p tutorial-api
//   curl localhost:3000/products
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{Value, json};
use tutorial::orders::{
    BulkDiscount, Inventory, Order, OrderError, OrderService, Product, Receipt,
};

// Everything the handlers share. Requests run on many threads at once, so
// anything that changes sits behind a Mutex, and the whole struct sits in
// an Arc so every handler can hold it.
struct AppState {
    catalog: BTreeMap<String, Product>,
    service: Mutex<OrderService<BulkDiscount>>,
    next_order: Mutex<u32>,
}

type SharedState = Arc<AppState>;

// The JSON body of POST /orders. Deserialize builds it from JSON.
#[derive(Deserialize)]
struct LineRequest {
    sku: String,
    quantity: u32,
}

#[derive(Deserialize)]
struct OrderRequest {
    customer: String,
    lines: Vec<LineRequest>,
}

#[tokio::main]
async fn main() {
    // 1. Build the shared state.
    let mut catalog = BTreeMap::new();
    for product in [
        Product::new("KB-01", "Keyboard", 45_00),
        Product::new("MS-02", "Mouse", 25_00),
        Product::new("MN-03", "Monitor", 180_00),
    ] {
        catalog.insert(product.sku.clone(), product);
    }
    let mut inventory = Inventory::new();
    for sku in catalog.keys() {
        inventory.restock(sku, 5);
    }
    let state = Arc::new(AppState {
        catalog,
        service: Mutex::new(OrderService::new(inventory, BulkDiscount::default())),
        next_order: Mutex::new(1000),
    });

    // 2. Map URLs to handler functions.
    let app = Router::new()
        .route("/products", get(list_products))
        .route("/stock/{sku}", get(stock_of))
        .route("/orders", post(create_order))
        .with_state(state);

    // 3. Listen and serve until stopped with Ctrl+C.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// axum fills each parameter from the request: State from the shared state,
// Path from the URL, Json from the request body.
async fn list_products(State(state): State<SharedState>) -> Json<Vec<Product>> {
    Json(state.catalog.values().cloned().collect())
}

async fn stock_of(State(state): State<SharedState>, Path(sku): Path<String>) -> Json<Value> {
    let in_stock = state.service.lock().unwrap().inventory().stock_of(&sku);
    Json(json!({ "sku": sku, "in_stock": in_stock }))
}

// Returning a Result: Ok becomes 200 with a JSON receipt, Err becomes the
// given status code with a JSON error message.
async fn create_order(
    State(state): State<SharedState>,
    Json(request): Json<OrderRequest>,
) -> Result<Json<Receipt>, (StatusCode, Json<Value>)> {
    let order_id = format!("ORD-{}", next_order_number(&state));

    let mut order = Order::new(&order_id, &request.customer);
    for line in &request.lines {
        let Some(product) = state.catalog.get(&line.sku) else {
            let error = json!({ "error": format!("unknown SKU {}", line.sku) });
            return Err((StatusCode::NOT_FOUND, Json(error)));
        };
        order
            .add(product.clone(), line.quantity)
            .map_err(bad_request)?;
    }

    let receipt = checkout(&state, &order).map_err(bad_request)?;
    Ok(Json(receipt))
}

// The handlers take a lock only as a temporary (stock_of) or inside these two
// plain (not async) functions, so no lock guard is ever held in a variable
// inside an async handler. Held across an .await, a std Mutex guard makes the
// handler's future unusable on other threads, and axum rejects it.
// rust-analyzer flags even a guard held in a variable with no .await,
// although cargo accepts that.
fn next_order_number(state: &AppState) -> u32 {
    let mut next = state.next_order.lock().unwrap();
    *next += 1;
    *next
}

fn checkout(state: &AppState, order: &Order) -> Result<Receipt, OrderError> {
    state.service.lock().unwrap().checkout(order)
}

fn bad_request(error: OrderError) -> (StatusCode, Json<Value>) {
    (
        StatusCode::BAD_REQUEST,
        Json(json!({ "error": error.to_string() })),
    )
}
