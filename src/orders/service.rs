//! Inventory and checkout.

use std::collections::HashMap;

use super::discounts::DiscountPolicy;
use super::models::{Order, OrderError, Receipt};
use crate::money::percent_of;

pub const TAX_RATE_PERCENT: i64 = 8;

/// How many of each product are on the shelf.
#[derive(Debug, Default)]
pub struct Inventory {
    stock: HashMap<String, u32>,
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            stock: HashMap::new(),
        }
    }

    pub fn restock(&mut self, sku: &str, quantity: u32) {
        *self.stock.entry(sku.to_string()).or_insert(0) += quantity;
    }

    // copied() turns Option<&u32> into Option<u32>; a missing sku has 0
    pub fn stock_of(&self, sku: &str) -> u32 {
        self.stock.get(sku).copied().unwrap_or(0)
    }

    fn remove(&mut self, sku: &str, quantity: u32) {
        if let Some(count) = self.stock.get_mut(sku) {
            *count -= quantity;
        }
    }
}

/// Turns an Order into a Receipt. Generic over the discount rule D:
/// the caller chooses the rule, and the service never names a concrete one.
pub struct OrderService<D: DiscountPolicy> {
    inventory: Inventory,
    discounts: D,
}

impl<D: DiscountPolicy> OrderService<D> {
    // Dependency injection: the service receives its collaborators
    // instead of creating them. Tests pass in whatever they need.
    pub fn new(inventory: Inventory, discounts: D) -> OrderService<D> {
        OrderService {
            inventory,
            discounts,
        }
    }

    pub fn inventory(&self) -> &Inventory {
        &self.inventory
    }

    pub fn inventory_mut(&mut self) -> &mut Inventory {
        &mut self.inventory
    }

    pub fn checkout(&mut self, order: &Order) -> Result<Receipt, OrderError> {
        if order.lines.is_empty() {
            return Err(OrderError::Empty {
                order_id: order.id.clone(),
            });
        }

        // Check every line first, so a failure leaves the stock untouched.
        for line in &order.lines {
            let available = self.inventory.stock_of(&line.product.sku);
            if available < line.quantity {
                return Err(OrderError::OutOfStock {
                    order_id: order.id.clone(),
                    name: line.product.name.clone(),
                    available,
                    requested: line.quantity,
                });
            }
        }

        for line in &order.lines {
            self.inventory.remove(&line.product.sku, line.quantity);
        }

        let subtotal = order.subtotal();
        let discount = self.discounts.discount_for(order);
        let tax = percent_of(subtotal - discount, TAX_RATE_PERCENT);
        Ok(Receipt {
            order_id: order.id.clone(),
            subtotal_cents: subtotal,
            discount_cents: discount,
            tax_cents: tax,
            total_cents: subtotal - discount + tax,
        })
    }
}
