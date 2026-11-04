//! The data types for the order system. Money is in cents (see crate::money).

use serde::Serialize;
use std::fmt;

/// One item for sale. Serialize lets the web API send it as JSON.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Product {
    pub sku: String,
    pub name: String,
    pub price_cents: i64,
}

impl Product {
    pub fn new(sku: &str, name: &str, price_cents: i64) -> Product {
        Product {
            sku: sku.to_string(),
            name: name.to_string(),
            price_cents,
        }
    }
}

/// One row of an order: which product, how many.
#[derive(Debug, Clone, PartialEq)]
pub struct OrderLine {
    pub product: Product,
    pub quantity: u32,
}

impl OrderLine {
    pub fn subtotal(&self) -> i64 {
        self.product.price_cents * self.quantity as i64
    }
}

/// An order being built up, line by line.
#[derive(Debug, Clone)]
pub struct Order {
    pub id: String,
    pub customer: String,
    pub lines: Vec<OrderLine>,
}

impl Order {
    pub fn new(id: &str, customer: &str) -> Order {
        Order {
            id: id.to_string(),
            customer: customer.to_string(),
            lines: Vec::new(),
        }
    }

    pub fn add(&mut self, product: Product, quantity: u32) -> Result<(), OrderError> {
        if quantity == 0 {
            return Err(OrderError::ZeroQuantity {
                order_id: self.id.clone(),
            });
        }
        self.lines.push(OrderLine { product, quantity });
        Ok(())
    }

    pub fn subtotal(&self) -> i64 {
        self.lines.iter().map(|line| line.subtotal()).sum()
    }
}

/// What checkout returns.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Receipt {
    pub order_id: String,
    pub subtotal_cents: i64,
    pub discount_cents: i64,
    pub tax_cents: i64,
    pub total_cents: i64,
}

/// Every way an order can fail. Callers match on the variant.
#[derive(Debug, Clone, PartialEq)]
pub enum OrderError {
    Empty {
        order_id: String,
    },
    ZeroQuantity {
        order_id: String,
    },
    OutOfStock {
        order_id: String,
        name: String,
        available: u32,
        requested: u32,
    },
}

impl fmt::Display for OrderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OrderError::Empty { order_id } => write!(f, "{order_id}: order is empty"),
            OrderError::ZeroQuantity { order_id } => {
                write!(f, "{order_id}: quantity must be at least 1")
            }
            OrderError::OutOfStock {
                order_id,
                name,
                available,
                requested,
            } => write!(
                f,
                "{order_id}: only {available} of {name} in stock, {requested} requested"
            ),
        }
    }
}

impl std::error::Error for OrderError {}
