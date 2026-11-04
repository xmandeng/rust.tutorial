//! Chapter 8: enums and pattern matching.

use std::f64::consts::PI;

/// An enum lists every form a value can take. Each variant can carry data.
#[derive(Debug)]
pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Square(f64),
}

impl Shape {
    pub fn area(&self) -> f64 {
        // match must handle every variant, or the build fails
        match self {
            Shape::Circle { radius } => PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Square(side) => side * side,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Shape::Circle { .. } => "Circle",
            Shape::Rectangle { .. } => "Rectangle",
            Shape::Square(_) => "Square",
        }
    }
}

/// Variants with no data work like Python's enum.Enum.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Light {
    Red,
    Yellow,
    Green,
}

impl Light {
    pub fn next(self) -> Light {
        match self {
            Light::Red => Light::Green,
            Light::Green => Light::Yellow,
            Light::Yellow => Light::Red,
        }
    }
}

// Patterns can test shapes, bind names and add conditions ("guards")
fn describe(point: (i32, i32)) -> String {
    match point {
        (0, 0) => "the origin".to_string(),
        (x, 0) => format!("on the x axis at {x}"),
        (0, y) => format!("on the y axis at {y}"),
        (x, y) if x == y => format!("on the diagonal at {x}"),
        (x, y) => format!("at ({x}, {y})"),
    }
}

// Option<T> is an ordinary enum: Some(value) or None
fn lookup(id: u32) -> Option<String> {
    match id {
        1 => Some("Ada".to_string()),
        2 => Some("Grace".to_string()),
        _ => None,
    }
}

pub fn run() {
    let shapes = vec![
        Shape::Circle { radius: 1.0 },
        Shape::Rectangle {
            width: 2.0,
            height: 3.0,
        },
        Shape::Square(2.0),
    ];
    let mut total = 0.0;
    for shape in &shapes {
        println!("{} with area {:.2}", shape.name(), shape.area());
        total += shape.area();
    }
    println!("Total area: {total:.2}");

    let mut light = Light::Red;
    for _ in 0..3 {
        print!("{light:?} -> ");
        light = light.next();
    }
    println!("{light:?}");

    for point in [(0, 0), (5, 0), (0, -2), (3, 3), (1, 2)] {
        println!("{point:?} is {}", describe(point));
    }

    for id in [1, 3] {
        match lookup(id) {
            Some(name) => println!("user {id} is {name}"),
            None => println!("user {id} not found"),
        }
    }
    // if let: match one pattern and ignore the rest
    if let Some(name) = lookup(2) {
        println!("found {name} with if let");
    }
}
