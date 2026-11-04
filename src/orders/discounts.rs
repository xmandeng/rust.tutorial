//! Discount rules. OrderService works with any type that implements
//! DiscountPolicy, so a new rule needs no change to the service.

use super::models::Order;
use crate::money::percent_of;

pub trait DiscountPolicy {
    /// The discount for this order, in cents.
    fn discount_for(&self, order: &Order) -> i64;
}

pub struct NoDiscount;

impl DiscountPolicy for NoDiscount {
    fn discount_for(&self, _order: &Order) -> i64 {
        0
    }
}

/// A percentage off any order whose subtotal reaches a threshold.
pub struct BulkDiscount {
    pub threshold_cents: i64,
    pub percent: i64,
}

impl BulkDiscount {
    pub fn new(threshold_cents: i64, percent: i64) -> BulkDiscount {
        BulkDiscount {
            threshold_cents,
            percent,
        }
    }
}

// Default is Rust's version of default arguments: BulkDiscount::default()
impl Default for BulkDiscount {
    fn default() -> BulkDiscount {
        BulkDiscount::new(100_00, 10)
    }
}

impl DiscountPolicy for BulkDiscount {
    fn discount_for(&self, order: &Order) -> i64 {
        let subtotal = order.subtotal();
        if subtotal < self.threshold_cents {
            return 0;
        }
        percent_of(subtotal, self.percent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::orders::models::Product;

    fn order_worth(cents: i64) -> Order {
        let mut order = Order::new("T", "Test");
        order.add(Product::new("X", "Thing", cents), 1).unwrap();
        order
    }

    #[test]
    fn bulk_discount_below_threshold_is_zero() {
        assert_eq!(BulkDiscount::default().discount_for(&order_worth(99_99)), 0);
    }

    #[test]
    fn bulk_discount_at_threshold_applies() {
        assert_eq!(
            BulkDiscount::default().discount_for(&order_worth(100_00)),
            10_00
        );
    }
}
