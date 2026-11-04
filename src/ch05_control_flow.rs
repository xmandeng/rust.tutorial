//! Chapter 5: control flow.

pub fn run() {
    // if is an expression, so it produces a value (Python: a if cond else b)
    let temperature = 23;
    let feel = if temperature > 25 {
        "hot"
    } else if temperature > 15 {
        "mild"
    } else {
        "cold"
    };
    println!("{temperature} degrees is {feel}");

    for score in [95, 82, 71, 40] {
        println!("{score} -> {}", grade(score));
    }

    // 1..=5 includes 5; 1..5 does not (Python: range(1, 6) and range(1, 5))
    let mut total = 0;
    for n in 1..=5 {
        total += n;
    }
    println!("sum of 1..=5 is {total}");

    // enumerate gives (index, item) pairs, as in Python
    let languages = ["Python", "Rust", "SQL"];
    for (i, language) in languages.iter().enumerate() {
        println!("{i}: {language}");
    }

    let mut n = 27;
    let mut steps = 0;
    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        steps += 1;
    }
    println!("27 reaches 1 in {steps} steps");

    // loop repeats until break, and break can hand back a value
    let mut guess = 0;
    let first_big_square = loop {
        guess += 1;
        if guess * guess > 50 {
            break guess;
        }
    };
    println!("first number whose square passes 50: {first_big_square}");
}

// match must cover every possible value; the compiler checks
fn grade(score: u32) -> char {
    match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        _ => 'F',
    }
}
