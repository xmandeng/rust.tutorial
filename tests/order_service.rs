use tutorial::orders::{
    BulkDiscount, DiscountPolicy, Inventory, NoDiscount, Order, OrderError, OrderService, Product,
};

fn mouse() -> Product {
    Product::new("MS-02", "Mouse", 25_00)
}

fn monitor() -> Product {
    Product::new("MN-03", "Monitor", 180_00)
}

// Helper: a service with 5 of each product and the given discount rule
fn build<D: DiscountPolicy>(discounts: D) -> OrderService<D> {
    let mut stock = Inventory::new();
    stock.restock("MS-02", 5);
    stock.restock("MN-03", 5);
    OrderService::new(stock, discounts)
}

#[test]
fn checkout_small_order_adds_tax_only() {
    let mut service = build(BulkDiscount::default());
    let mut order = Order::new("T-1", "Ada");
    order.add(mouse(), 2).unwrap(); // 50.00

    let receipt = service.checkout(&order).unwrap();

    assert_eq!(receipt.discount_cents, 0);
    assert_eq!(receipt.tax_cents, 4_00); // 8% of 50.00
    assert_eq!(receipt.total_cents, 54_00);
}

#[test]
fn checkout_large_order_gets_bulk_discount() {
    let mut service = build(BulkDiscount::new(100_00, 10));
    let mut order = Order::new("T-2", "Grace");
    order.add(monitor(), 1).unwrap(); // 180.00

    let receipt = service.checkout(&order).unwrap();

    assert_eq!(receipt.discount_cents, 18_00);
    assert_eq!(receipt.total_cents, 174_96); // (180 - 18) * 1.08
}

#[test]
fn checkout_not_enough_stock_fails_and_leaves_stock_alone() {
    let mut service = build(NoDiscount);
    let mut order = Order::new("T-3", "Linus");
    order.add(mouse(), 1).unwrap();
    order.add(monitor(), 99).unwrap();

    let result = service.checkout(&order);

    assert!(matches!(
        result,
        Err(OrderError::OutOfStock { available: 5, .. })
    ));
    assert_eq!(service.inventory().stock_of("MS-02"), 5); // the mouse was not removed
}

#[test]
fn checkout_empty_order_fails() {
    let mut service = build(NoDiscount);
    let result = service.checkout(&Order::new("T-4", "Alan"));
    assert_eq!(
        result,
        Err(OrderError::Empty {
            order_id: "T-4".to_string()
        })
    );
}
