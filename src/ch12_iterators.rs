//! Chapter 12: iterators and closures, Rust's list comprehensions.

/// One member of staff.
#[derive(Debug, Clone)]
pub struct Employee {
    pub name: String,
    pub dept: String,
    pub salary: u32,
    pub years: u32,
}

impl Employee {
    pub fn new(name: &str, dept: &str, salary: u32, years: u32) -> Employee {
        Employee {
            name: name.to_string(),
            dept: dept.to_string(),
            salary,
            years,
        }
    }
}

pub fn staff() -> Vec<Employee> {
    vec![
        Employee::new("Ada", "Engineering", 120_000, 5),
        Employee::new("Grace", "Engineering", 135_000, 9),
        Employee::new("Linus", "Engineering", 98_000, 2),
        Employee::new("Margaret", "Sales", 87_000, 4),
        Employee::new("Alan", "Sales", 91_000, 0),
        Employee::new("Barbara", "Finance", 105_000, 7),
    ]
}

pub fn run() {
    // A closure is an anonymous function (Python: lambda). It can use
    // variables from the surrounding code.
    let bonus = 500;
    let with_bonus = |salary: u32| salary + bonus;
    println!("with bonus: {}", with_bonus(1_000));

    let staff = staff();

    // Python: [e.name for e in staff if e.salary > 100_000]
    let high_earners: Vec<&str> = staff
        .iter()
        .filter(|e| e.salary > 100_000)
        .map(|e| e.name.as_str())
        .collect();
    println!("High earners: {}", high_earners.join(", "));

    // Aggregates
    let payroll: u32 = staff.iter().map(|e| e.salary).sum();
    let average_years = staff.iter().map(|e| e.years).sum::<u32>() as f64 / staff.len() as f64;
    let any_new = staff.iter().any(|e| e.years == 0);
    let in_hr = staff.iter().find(|e| e.dept == "HR");
    println!("Payroll {payroll}, avg years {average_years:.1}, new hires? {any_new}");
    println!("HR: {:?}", in_hr.map(|e| &e.name));

    // Python: sorted(staff, key=lambda e: (-e.years, e.name))
    let mut by_seniority = staff.clone();
    by_seniority.sort_by(|a, b| b.years.cmp(&a.years).then(a.name.cmp(&b.name)));
    println!("Most senior: {}", by_seniority[0].name);

    let max_paid = staff.iter().max_by_key(|e| e.salary).unwrap();
    println!("Top salary: {} ({})", max_paid.name, max_paid.salary);

    // Iterators are lazy: nothing runs until something consumes them.
    // Python generators behave the same way.
    let squares = (1..).map(|n| n * n).filter(|n| n % 2 == 1);
    let first_four: Vec<u64> = squares.take(4).collect();
    println!("first four odd squares: {first_four:?}");

    // zip, enumerate and rev work as in Python
    let names = ["Ada", "Grace"];
    let langs = ["Analytical Engine", "COBOL"];
    for (name, lang) in names.iter().zip(langs.iter()) {
        println!("{name} -> {lang}");
    }
    let countdown: Vec<String> = (1..=3).rev().map(|n| n.to_string()).collect();
    println!("{}", countdown.join("..."));
}
