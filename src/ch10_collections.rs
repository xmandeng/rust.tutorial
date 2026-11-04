//! Chapter 10: collections.

use std::collections::{BTreeMap, HashMap, HashSet};

pub fn run() {
    // Vec<T>: a growable list (Python: list). vec! builds one from values.
    let mut languages = vec!["Python", "Rust", "Go"];
    languages.push("SQL");
    languages.sort();
    println!("{languages:?} has {} items", languages.len());
    println!(
        "first {:?}, tenth {:?}",
        languages.first(),
        languages.get(9)
    );
    println!("contains Rust? {}", languages.contains(&"Rust"));

    // A slice borrows part of a Vec (Python: languages[1:3], but no copy)
    let middle: &[&str] = &languages[1..3];
    println!("middle {middle:?}");

    // HashMap<K, V>: Python's dict
    let mut stock: HashMap<String, u32> = HashMap::new();
    stock.insert("apple".to_string(), 3);
    stock.insert("pear".to_string(), 5);
    // entry().or_insert() is Python's dict.setdefault
    *stock.entry("apple".to_string()).or_insert(0) += 2;
    println!("apples: {}", stock["apple"]);
    match stock.get("plum") {
        Some(count) => println!("plums: {count}"),
        None => println!("no plums"),
    }

    // HashMap iteration order is random, run to run. BTreeMap keeps keys
    // sorted, so use it (or sort the keys) whenever the order is shown.
    let text = "the cat sat on the mat and the dog sat too";
    let mut counts: BTreeMap<&str, u32> = BTreeMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    println!("{counts:?}");

    // HashSet<T>: Python's set
    let rust_devs = HashSet::from(["Ada", "Grace", "Linus"]);
    let python_devs = HashSet::from(["Grace", "Guido", "Linus"]);
    // copied() turns the &&str items intersection yields into plain &str
    let mut both: Vec<&str> = rust_devs.intersection(&python_devs).copied().collect();
    both.sort();
    println!("know both: {both:?}");
    println!("Guido knows Rust? {}", rust_devs.contains("Guido"));

    // Iterating: & borrows each item; without & the loop consumes the Vec
    let mut prices = vec![100, 250, 75];
    for price in &mut prices {
        *price += 10; // * reaches through the reference to change the item
    }
    for price in &prices {
        print!("{price} ");
    }
    println!();
}
