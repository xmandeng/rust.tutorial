//! Chapter 3: syntax basics.

pub fn run() {
    // let makes a variable. Types are inferred: year is an i32.
    let name = "Ada";
    let year = 1843;
    println!("{name} published the first program in {year}.");

    // Variables are immutable unless declared with mut
    let mut count = 0;
    count += 1;
    println!("count is {count}");

    // {:.2} is Python's :.2f, {:>8} right-aligns, {:?} is debug output (like repr)
    let price = 19.987;
    println!("price {price:.2}, padded [{year:>8}], debug {name:?}");

    // format! builds a String instead of printing it (Python: an f-string)
    let greeting = format!("Hello, {name}!");
    // len() counts bytes, not characters (they match for plain ASCII text)
    println!("{greeting} is {} bytes long", greeting.len());

    // Shadowing: a new let with the same name replaces the old variable
    let year = year + 1;
    println!("a year later it was {year}");

    // A block is an expression: its last line, with no semicolon, is its value
    let area = {
        let width = 3;
        let height = 4;
        width * height
    };
    println!("area is {area}");

    println!("square(7) is {}", square(7));
}

// Parameter and return types are required. The last expression is returned.
fn square(n: i32) -> i32 {
    n * n
}
