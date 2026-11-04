//! Chapter 9: error handling with Result and Option instead of exceptions.

use std::fmt;
use std::num::ParseIntError;

// The long way: match on the Result that parse returns
fn parse_age(text: &str) -> Result<u32, String> {
    match text.trim().parse::<u32>() {
        Ok(age) => Ok(age),
        Err(e) => Err(format!("'{text}' is not an age: {e}")),
    }
}

// The short way: `?` returns the error to the caller, or unwraps the value
fn sum_numbers(text: &str) -> Result<i64, ParseIntError> {
    let mut total = 0;
    for part in text.split(',') {
        total += part.trim().parse::<i64>()?;
    }
    Ok(total)
}

/// A custom error type: one enum variant per way the operation can fail.
#[derive(Debug)]
pub enum ConfigError {
    Missing(String),
    NotANumber { key: String, value: String },
    OutOfRange { key: String, value: i64 },
}

// Display gives the error its human-readable message (Python: __str__)
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::Missing(key) => write!(f, "setting '{key}' is missing"),
            ConfigError::NotANumber { key, value } => {
                write!(f, "setting '{key}' should be a number, got '{value}'")
            }
            ConfigError::OutOfRange { key, value } => {
                write!(f, "setting '{key}' is out of range: {value}")
            }
        }
    }
}

// Marks ConfigError as an error type that other error-handling code accepts
impl std::error::Error for ConfigError {}

// Find `key` in text made of "key=value" lines.
fn find_setting(text: &str, key: &str) -> Option<String> {
    for line in text.lines() {
        // if let ... && ...: the pattern must match and the condition hold
        if let Some((k, v)) = line.split_once('=')
            && k.trim() == key
        {
            return Some(v.trim().to_string());
        }
    }
    None
}

/// Read a TCP port from config text, explaining what is wrong if it can't.
pub fn read_port(config: &str) -> Result<u16, ConfigError> {
    // ok_or turns an Option into a Result: None becomes the given error
    let raw = find_setting(config, "port").ok_or(ConfigError::Missing("port".to_string()))?;
    let value: i64 = match raw.parse() {
        Ok(n) => n,
        Err(_) => {
            return Err(ConfigError::NotANumber {
                key: "port".to_string(),
                value: raw,
            });
        }
    };
    if !(1..=65535).contains(&value) {
        return Err(ConfigError::OutOfRange {
            key: "port".to_string(),
            value,
        });
    }
    Ok(value as u16)
}

pub fn run() {
    println!("{:?}", parse_age("36"));
    println!("{:?}", parse_age("abc"));

    println!("{:?}", sum_numbers("1, 2, 3"));
    match sum_numbers("1, two, 3") {
        Ok(total) => println!("total {total}"),
        Err(e) => println!("could not sum: {e}"),
    }

    let configs = ["host=db\nport=5432", "host=db", "port=http", "port=70000"];
    for config in configs {
        match read_port(config) {
            Ok(port) => println!("port {port}"),
            Err(e) => println!("config error: {e}"),
        }
    }

    // Shortcuts. unwrap and expect panic (crash) on an error, so they belong
    // in tests and quick scripts, not in code that should keep running.
    let port = read_port("port=8080").expect("config should be valid");
    let fallback = read_port("").unwrap_or(3000);
    println!("port {port}, fallback {fallback}");

    // A panic is for bugs, not expected failures. Indexing past the end panics;
    // .get returns an Option instead.
    let items = [10, 20, 30];
    println!("get(1) = {:?}, get(9) = {:?}", items.get(1), items.get(9));
}
