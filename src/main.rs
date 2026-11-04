// Entry point: runs the demo for one book chapter, or for all of them.
//   cargo run            every chapter
//   cargo run -- 6       only chapter 6
use std::env;

use tutorial::{
    ch03_basics, ch04_types, ch05_control_flow, ch06_ownership, ch07_structs, ch08_enums,
    ch09_errors, ch10_collections, ch11_traits, ch12_iterators, ch13_lifetimes, ch14_concurrency,
    ch15_orders,
};

fn main() {
    let titles = [
        (3, "Syntax basics"),
        (4, "Types and variables"),
        (5, "Control flow"),
        (6, "Ownership and borrowing"),
        (7, "Structs and methods"),
        (8, "Enums and pattern matching"),
        (9, "Error handling"),
        (10, "Collections"),
        (11, "Traits and generics"),
        (12, "Iterators and closures"),
        (13, "Lifetimes and smart pointers"),
        (14, "Concurrency and async"),
        (15, "The order system"),
    ];

    // env::args() is Python's sys.argv; skip(1) drops the program name
    let args: Vec<String> = env::args().skip(1).collect();
    let mut chapters: Vec<u32> = Vec::new();
    if args.is_empty() {
        for (number, _) in titles {
            chapters.push(number);
        }
    } else {
        for arg in &args {
            chapters.push(arg.parse().expect("chapter must be a number"));
        }
    }

    for chapter in chapters {
        let mut title = "unknown";
        for (number, name) in titles {
            if number == chapter {
                title = name;
            }
        }
        println!("\n=== Chapter {chapter}: {title} ===");
        match chapter {
            3 => ch03_basics::run(),
            4 => ch04_types::run(),
            5 => ch05_control_flow::run(),
            6 => ch06_ownership::run(),
            7 => ch07_structs::run(),
            8 => ch08_enums::run(),
            9 => ch09_errors::run(),
            10 => ch10_collections::run(),
            11 => ch11_traits::run(),
            12 => ch12_iterators::run(),
            13 => ch13_lifetimes::run(),
            14 => ch14_concurrency::run(),
            15 => ch15_orders::run(),
            _ => println!("No demo for chapter {chapter}. Valid chapters: 3 to 15."),
        }
    }
}
