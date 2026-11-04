//! The order system: products, orders, discounts and checkout.
//!
//! `models` holds the data types, `discounts` the pluggable discount rules,
//! `service` the inventory and the checkout logic. The `pub use` lines let
//! callers write `tutorial::orders::Order` instead of the full path.

pub mod discounts;
pub mod models;
pub mod service;

pub use discounts::{BulkDiscount, DiscountPolicy, NoDiscount};
pub use models::{Order, OrderError, OrderLine, Product, Receipt};
pub use service::{Inventory, OrderService, TAX_RATE_PERCENT};
