// Integration tests: each file in tests/ is compiled as a separate program
// that uses the library the way any outside code would.
use tutorial::ch07_structs::BankAccount;

#[test]
fn new_account_starts_empty() {
    let account = BankAccount::new("ACC-1", "Ada");
    assert_eq!(account.balance(), 0);
    assert_eq!(account.owner(), "Ada");
}

#[test]
fn deposit_adds_to_balance() {
    let mut account = BankAccount::new("ACC-1", "Ada");
    account.deposit(50_00).unwrap();
    account.deposit(25_00).unwrap();
    assert_eq!(account.balance(), 75_00);
}

#[test]
fn deposit_zero_is_rejected() {
    let mut account = BankAccount::new("ACC-1", "Ada");
    let result = account.deposit(0);
    assert!(result.is_err());
    assert_eq!(account.balance(), 0);
}

#[test]
fn withdraw_more_than_balance_is_rejected() {
    let mut account = BankAccount::new("ACC-1", "Ada");
    account.deposit(100_00).unwrap();
    let result = account.withdraw(150_00);
    assert_eq!(
        result,
        Err("insufficient funds: balance 100.00, requested 150.00".to_string())
    );
    assert_eq!(account.balance(), 100_00); // unchanged
}
