//! Chapter 11: traits and generics.

use std::fmt;
use std::ops::Add;

/// A trait lists methods a type must provide (Python: an ABC or a Protocol).
pub trait Describe {
    fn name(&self) -> String;

    // A default method: implementing types get it free and may replace it
    fn describe(&self) -> String {
        format!("This is {}", self.name())
    }
}

pub struct Dog {
    pub name: String,
}

pub struct Robot {
    pub model: u32,
}

impl Describe for Dog {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn describe(&self) -> String {
        format!("{} says woof", self.name)
    }
}

impl Describe for Robot {
    fn name(&self) -> String {
        format!("Robot #{}", self.model)
    }
}

// Generic function: T is any type that implements Describe. The compiler
// writes a separate copy for each T used ("static dispatch").
pub fn announce<T: Describe>(item: &T) -> String {
    format!("Announcing: {}", item.describe())
}

// Generic with two trait bounds: works for i32, f64, char, ...
pub fn largest<T: PartialOrd + Copy>(items: &[T]) -> T {
    let mut best = items[0];
    for &item in items {
        if item > best {
            best = item;
        }
    }
    best
}

/// Implementing standard-library traits gives a type built-in behaviour.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fraction {
    pub num: i64,
    pub den: i64,
}

impl Fraction {
    pub fn new(num: i64, den: i64) -> Fraction {
        let divisor = gcd(num.abs(), den.abs());
        Fraction {
            num: num / divisor,
            den: den / divisor,
        }
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

// Display: what {} prints (Python: __str__)
impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

// Add: what + does (Python: __add__)
impl Add for Fraction {
    type Output = Fraction;

    fn add(self, other: Fraction) -> Fraction {
        Fraction::new(
            self.num * other.den + other.num * self.den,
            self.den * other.den,
        )
    }
}

pub fn run() {
    let rex = Dog {
        name: "Rex".to_string(),
    };
    let r2 = Robot { model: 2 };
    println!("{}", announce(&rex));
    println!("{}", announce(&r2));

    // A trait object (Box<dyn Describe>) lets one Vec hold different types.
    // The method to call is looked up at run time ("dynamic dispatch").
    let things: Vec<Box<dyn Describe>> = vec![
        Box::new(Dog {
            name: "Fido".to_string(),
        }),
        Box::new(Robot { model: 7 }),
    ];
    for thing in &things {
        println!("{}", thing.describe());
    }

    let biggest_int = largest(&[3, 9, 4]);
    let biggest_float = largest(&[1.5, 0.2]);
    let biggest_char = largest(&['x', 'b']);
    println!("largest: {biggest_int} {biggest_float} {biggest_char}");

    let half = Fraction::new(1, 2);
    let third = Fraction::new(1, 3);
    println!("{half} + {third} = {}", half + third);
    println!("debug: {:?}", half + half);
    println!("2/4 == 1/2? {}", Fraction::new(2, 4) == half);
}
