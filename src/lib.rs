//! Companion code for the book "Rust for Python Developers".
//!
//! Each `chNN_*` module holds the demo for book chapter NN; `src/main.rs`
//! runs them. The `orders` module is the small order system that chapter 15
//! walks through, the tests in `tests/` check, and the web API in `api/` serves.

pub mod ch03_basics;
pub mod ch04_types;
pub mod ch05_control_flow;
pub mod ch06_ownership;
pub mod ch07_structs;
pub mod ch08_enums;
pub mod ch09_errors;
pub mod ch10_collections;
pub mod ch11_traits;
pub mod ch12_iterators;
pub mod ch13_lifetimes;
pub mod ch14_concurrency;
pub mod ch15_orders;
pub mod money;
pub mod orders;
