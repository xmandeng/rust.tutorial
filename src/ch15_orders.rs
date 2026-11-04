//! Chapter 15: a small program that uses everything so far.

use crate::money::dollars;
use crate::orders::{BulkDiscount, Inventory, Order, OrderService, Product, Receipt};

pub fn run() {
    let keyboard = Product::new("KB-01", "Keyboard", 45_00);
    let mouse = Product::new("MS-02", "Mouse", 25_00);
    let monitor = Product::new("MN-03", "Monitor", 180_00);

    let mut inventory = Inventory::new();
    inventory.restock(&keyboard.sku, 10);
    inventory.restock(&mouse.sku, 10);
    inventory.restock(&monitor.sku, 1);

    // The service takes ownership of the inventory: it is moved in here.
    let mut service = OrderService::new(inventory, BulkDiscount::default());

    let mut small = Order::new("ORD-1", "Ada");
    small.add(mouse.clone(), 2).unwrap();
    print_receipt(&service.checkout(&small).unwrap());

    let mut large = Order::new("ORD-2", "Grace");
    large.add(keyboard.clone(), 1).unwrap();
    large.add(monitor.clone(), 1).unwrap();
    print_receipt(&service.checkout(&large).unwrap());

    let mut too_many = Order::new("ORD-3", "Linus");
    too_many.add(monitor.clone(), 1).unwrap(); // the only monitor went in ORD-2
    match service.checkout(&too_many) {
        Ok(receipt) => print_receipt(&receipt),
        Err(e) => println!("Rejected: {e}"),
    }

    println!("Mice left: {}", service.inventory().stock_of(&mouse.sku));
}

fn print_receipt(r: &Receipt) {
    println!(
        "{}: subtotal {} - discount {} + tax {} = {}",
        r.order_id,
        dollars(r.subtotal_cents),
        dollars(r.discount_cents),
        dollars(r.tax_cents),
        dollars(r.total_cents)
    );
}
