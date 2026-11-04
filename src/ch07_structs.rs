//! Chapter 7: structs and methods.

use crate::money::dollars;

/// A bank account. The fields are private, so code outside this module
/// can change the balance only through the methods below.
#[derive(Debug)]
pub struct BankAccount {
    id: String,
    owner: String,
    balance_cents: i64,
}

impl BankAccount {
    /// An associated function (no self). `new` is the constructor by convention.
    pub fn new(id: &str, owner: &str) -> BankAccount {
        BankAccount {
            id: id.to_string(),
            owner: owner.to_string(),
            balance_cents: 0,
        }
    }

    // &self: the method only reads the account
    pub fn balance(&self) -> i64 {
        self.balance_cents
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    // &mut self: the method changes the account
    pub fn deposit(&mut self, cents: i64) -> Result<(), String> {
        if cents <= 0 {
            return Err(format!("deposit must be positive, got {cents}"));
        }
        self.balance_cents += cents;
        Ok(())
    }

    pub fn withdraw(&mut self, cents: i64) -> Result<(), String> {
        if cents > self.balance_cents {
            return Err(format!(
                "insufficient funds: balance {}, requested {}",
                dollars(self.balance_cents),
                dollars(cents)
            ));
        }
        self.balance_cents -= cents;
        Ok(())
    }

    pub fn summary(&self) -> String {
        format!(
            "{} {}: {}",
            self.id,
            self.owner,
            dollars(self.balance_cents)
        )
    }
}

/// A plain data struct. derive generates code for printing ({:?}),
/// copying and comparing with ==, as @dataclass(frozen=True) does.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance_to(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

/// A tuple struct: a named wrapper around one value (a "newtype").
pub struct Celsius(pub f64);

impl Celsius {
    pub fn to_fahrenheit(&self) -> f64 {
        self.0 * 9.0 / 5.0 + 32.0
    }
}

pub fn run() {
    let mut account = BankAccount::new("ACC-0001", "Ada");
    account.deposit(120_00).unwrap();
    println!("{}", account.summary());

    match account.withdraw(500_00) {
        Ok(()) => println!("withdrew 500.00"),
        Err(message) => println!("Refused: {message}"),
    }
    if let Err(message) = account.deposit(-5) {
        println!("Refused: {message}");
    }
    account.withdraw(20_00).unwrap();
    println!("{} now has {}", account.owner(), dollars(account.balance()));
    println!("{account:?}");

    let origin = Point { x: 0.0, y: 0.0 };
    let p = Point { x: 3.0, y: 4.0 };
    println!("{origin:?} to {p:?} is {}", origin.distance_to(&p));
    println!("equal by value: {}", p == Point { x: 3.0, y: 4.0 });

    let body = Celsius(37.0);
    println!("{} C is {} F", body.0, body.to_fahrenheit());
}
