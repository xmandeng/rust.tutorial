//! Money is stored as a whole number of cents (`i64`), never as a float.
//! Rust has no built-in decimal type; integer cents are the usual answer
//! (the `rust_decimal` crate is the other).

/// Format cents as dollars with two decimal places: 12345 -> "123.45".
pub fn dollars(cents: i64) -> String {
    let sign = if cents < 0 { "-" } else { "" };
    let abs = cents.abs();
    format!("{sign}{}.{:02}", abs / 100, abs % 100)
}

/// `percent` percent of `cents`, rounded to the nearest cent.
pub fn percent_of(cents: i64, percent: i64) -> i64 {
    (cents * percent + 50) / 100
}

// Unit tests live next to the code, in a module compiled only by `cargo test`.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dollars_pads_cents() {
        assert_eq!(dollars(12345), "123.45");
        assert_eq!(dollars(5), "0.05");
        assert_eq!(dollars(-250), "-2.50");
    }

    #[test]
    fn percent_of_rounds_to_nearest_cent() {
        assert_eq!(percent_of(20250, 8), 1620); // 8% of 202.50 is 16.20
        assert_eq!(percent_of(1006, 10), 101); // 100.6 cents rounds up
    }
}
