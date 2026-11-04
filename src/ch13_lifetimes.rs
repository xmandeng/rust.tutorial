//! Chapter 13: lifetimes, Drop, and smart pointers (Box, Rc, RefCell).

use std::cell::RefCell;
use std::rc::Rc;

// A lifetime ('a) tells the compiler how long a borrowed value must live.
// This says: the result borrows from x or y, so it is valid only while
// both of them are.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

/// A struct that holds a reference needs a lifetime too: a Headline
/// may not outlive the text it points into.
pub struct Headline<'a> {
    pub text: &'a str,
}

impl<'a> Headline<'a> {
    pub fn first_word(&self) -> &'a str {
        self.text.split_whitespace().next().unwrap_or("")
    }
}

/// Drop runs when a value goes out of scope, like a `with` block's
/// __exit__. Files, locks and sockets close themselves this way (RAII).
pub struct Resource {
    pub name: String,
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("  closing {}", self.name);
    }
}

/// Box<T> puts a value on the heap. A recursive type needs it, because
/// otherwise its size would be infinite.
pub enum Expr {
    Number(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

pub fn eval(expr: &Expr) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
    }
}

pub fn run() {
    let a = String::from("a short line");
    let result;
    {
        let b = String::from("a much longer line");
        result = longest(&a, &b);
        println!("longest: {result}");
    } // b is dropped here, so `result` may not be used after this brace

    let article = String::from("Rust reaches version two");
    let headline = Headline { text: &article };
    println!("first word: {}", headline.first_word());

    println!("before the block");
    {
        let _file = Resource {
            name: "data.csv".to_string(),
        };
        let _conn = Resource {
            name: "database".to_string(),
        };
        println!("  using both");
    } // dropped in reverse order of creation
    println!("after the block");

    // (2 + 3) * 4 as a tree
    let expr = Expr::Mul(
        Box::new(Expr::Add(
            Box::new(Expr::Number(2)),
            Box::new(Expr::Number(3)),
        )),
        Box::new(Expr::Number(4)),
    );
    println!("(2 + 3) * 4 = {}", eval(&expr));

    // Rc<T>: several owners of one value, freed when the last owner goes.
    // RefCell<T>: allows changes through a shared reference, checked at run time.
    // Together they give Python-style shared, mutable objects.
    let shared = Rc::new(RefCell::new(vec![1, 2]));
    let other_owner = Rc::clone(&shared);
    other_owner.borrow_mut().push(3);
    println!(
        "shared = {:?}, owners = {}",
        shared.borrow(),
        Rc::strong_count(&shared)
    );
}
