//! Chapter 4: types and variables.

pub fn run() {
    // Scalar types. The annotations are optional here; they show the names.
    let age: i32 = 36;
    let population: i64 = 8_100_000_000;
    let ratio: f64 = 0.75;
    let ready: bool = true;
    let letter: char = 'R';
    println!("{age} {population} {ratio} {ready} {letter}");

    // Integer division truncates. 7 / 2.0 would not compile: no mixed types.
    println!(
        "7 / 2 = {}, 7.0 / 2.0 = {}, 7 % 2 = {}",
        7 / 2,
        7.0 / 2.0,
        7 % 2
    );

    // No automatic conversion between number types; `as` converts explicitly
    let scaled = age as f64 * ratio;
    println!("scaled = {scaled}");

    // Integers have a fixed size. A u8 holds 0 to 255.
    let small: u8 = 250;
    println!("checked_add: {:?}", small.checked_add(10));
    println!("wrapping_add: {}", small.wrapping_add(10));

    // Two string types. String owns its text and can grow;
    // &str ("string slice") borrows text that something else owns.
    let mut owned: String = String::from("hello");
    owned.push_str(", world");
    let borrowed: &str = &owned;
    let literal: &str = "literals are &str";
    println!("{borrowed} / {literal}");
    println!(
        "upper: {}, words: {}",
        owned.to_uppercase(),
        owned.split(", ").count()
    );

    // Tuples group values of different types; arrays have a fixed length
    let point: (i32, f64) = (3, 4.5);
    let (x, y) = point;
    let primes: [i32; 4] = [2, 3, 5, 7];
    println!(
        "x={x} y={y} first prime={} count={}",
        primes[0],
        primes.len()
    );

    // No null. A value that may be missing has type Option<T>: Some(value) or None.
    // find returns the position of a character, if the text contains it.
    let w_at: Option<usize> = owned.find('w');
    let z_at: Option<usize> = owned.find('z');
    println!("{w_at:?} {z_at:?}");
    println!(
        "z found? {}, z position or 0: {}",
        z_at.is_some(),
        z_at.unwrap_or(0)
    );
}
