//! Chapter 6: ownership and borrowing, the idea the rest of Rust is built on.

pub fn run() {
    // 1. Move: assigning a String moves it. The old name can no longer be used.
    let a = String::from("report.pdf");
    let b = a;
    // println!("{a}");   // error[E0382]: borrow of moved value: `a`
    println!("b = {b}");

    // 2. clone makes a real copy when you want two independent values
    let c = b.clone();
    println!("b = {b}, c = {c}");

    // 3. Small fixed-size types (numbers, bool, char) are Copy: copied, not moved
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    // 4. Passing a value to a function moves it into the function
    let title = String::from("Annual Report");
    let length = take_ownership(title);
    // println!("{title}");  // error[E0382]: title was moved
    println!("the function took a title of {length} bytes");

    // 5. Borrow: &doc lends read access; doc keeps ownership
    let doc = String::from("Quarterly Report for Q3");
    let words = count_words(&doc);
    println!("'{doc}' has {words} words");

    // 6. Mutable borrow: &mut log lends write access
    let mut log: Vec<String> = Vec::new();
    add_entry(&mut log, "started");
    add_entry(&mut log, "finished");
    println!("log = {log:?}");

    // 7. The borrow rule: any number of readers, or one writer, never both at once
    let mut scores = vec![10, 20, 30];
    let first = &scores[0];
    // scores.push(40);  // error[E0502]: cannot borrow `scores` as mutable
    //                   // because it is also borrowed as immutable
    println!("first = {first}");
    scores.push(40); // allowed: `first` is not used after this point
    println!("scores = {scores:?}");
} // every value still owned here is dropped (freed) at this brace

// Takes ownership of `text`; the String is freed when this function returns.
fn take_ownership(text: String) -> usize {
    text.len()
}

// Borrows read-only. &str accepts a &String as well as a literal.
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

// Borrows mutably, so it may change the caller's vector.
fn add_entry(log: &mut Vec<String>, entry: &str) {
    log.push(entry.to_string());
}
