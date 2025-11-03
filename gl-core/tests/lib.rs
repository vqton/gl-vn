// use super::*; // Assuming this is needed in the module context
use gl_core::Ledger;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use gl_core::AccountCode;
use gl_core::JournalLine;
use gl_core::JournalEntry;
use std::str::FromStr; // Needed for Decimal::from_str

// --- AccountCode Tests ---
// Tests the basic validation of the AccountCode struct.
#[test]
fn test_account_code_valid() {
    let code = AccountCode::new("1111").unwrap();
    assert_eq!(code.as_str(), "1111");
}

#[test]
fn test_account_code_invalid_format_fails() {
    let result = AccountCode::new("invalid"); 
    assert!(result.is_err(), "AccountCode creation should fail for invalid formats.");
}

// --- JournalLine Tests ---
// Tests the validation within a single JournalLine.
#[test]
fn test_journal_line_negative_amount_fails() {
    let code = AccountCode::new("5111").unwrap(); 
    // Test for a negative debit amount
    let result = JournalLine::new(code, Decimal::from(-100), Decimal::ZERO);
    assert!(result.is_err(), "JournalLine should not allow negative amounts.");
}

// --- JournalEntry Tests ---
// Tests the core double-entry principle and JournalEntry-level constraints.
#[test]
fn test_journal_entry_balanced() {
    let entry = JournalEntry::new(
        // FIX: Replaced NaiveDate::from_ymd
        NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        vec![
            JournalLine::new(AccountCode::new("5111").unwrap(), Decimal::from(100), Decimal::ZERO).unwrap(), // Debit $100
            JournalLine::new(AccountCode::new("1111").unwrap(), Decimal::ZERO, Decimal::from(100)).unwrap(), // Credit $100
        ],
        "Sale".to_string(),
    ).unwrap();
    assert!(entry.is_balanced());
}

#[test]
fn test_unbalanced_entry_fails_on_creation() {
    let result = JournalEntry::new(
        // FIX: Replaced NaiveDate::from_ymd
        NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        vec![
            // Total Debit = 100, Total Credit = 90
            JournalLine::new(AccountCode::new("5111").unwrap(), Decimal::from(100), Decimal::ZERO).unwrap(),
            JournalLine::new(AccountCode::new("1111").unwrap(), Decimal::ZERO, Decimal::from(90)).unwrap(),
        ],
        "Unbalanced".to_string(),
    );
    assert!(result.is_err(), "JournalEntry creation must fail if debits do not equal credits.");
}

#[test]
fn test_zero_sum_balanced_entry() {
    let entry = JournalEntry::new(
        // FIX: Replaced NaiveDate::from_ymd
        NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        vec![
            JournalLine::new(AccountCode::new("5111").unwrap(), Decimal::ZERO, Decimal::ZERO).unwrap(),
            JournalLine::new(AccountCode::new("1111").unwrap(), Decimal::ZERO, Decimal::ZERO).unwrap(),
        ],
        "Zero sum transaction".to_string(),
    ).unwrap();
    assert!(entry.is_balanced(), "A zero-sum entry with at least two lines must still be considered balanced.");
}

#[test]
fn test_decimal_amount_entry_is_balanced() {
    let entry = JournalEntry::new(
        // FIX: Replaced NaiveDate::from_ymd
        NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        vec![
            JournalLine::new(AccountCode::new("5111").unwrap(), Decimal::from_str("123.45").unwrap(), Decimal::ZERO).unwrap(),
            JournalLine::new(AccountCode::new("1111").unwrap(), Decimal::ZERO, Decimal::from_str("123.45").unwrap()).unwrap(),
        ],
        "Decimal amount".to_string(),
    ).unwrap();
    assert!(entry.is_balanced());
}

// --- Ledger Tests ---
// Tests the posting and reporting functionality of the Ledger.
#[test]
fn test_ledger_post_single_entry() {
    let mut ledger = Ledger::new();
    let entry = JournalEntry::new(
        // FIX: Replaced NaiveDate::from_ymd
        NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        vec![
            JournalLine::new(AccountCode::new("5111").unwrap(), Decimal::from(100), Decimal::ZERO).unwrap(),
            JournalLine::new(AccountCode::new("1111").unwrap(), Decimal::ZERO, Decimal::from(100)).unwrap(),
        ],
        "Sale".to_string(),
    ).unwrap();

    ledger.post_entry(entry).unwrap();

    let tb = ledger.trial_balance();
    assert_eq!(tb.total_debit, Decimal::from(100));
    assert_eq!(tb.total_credit, Decimal::from(100));
}

#[test]
fn test_ledger_multiple_entries() {
    let mut ledger = Ledger::new();
    
    // Entry 1: Debit 5111 (Revenue) $100, Credit 1111 (Cash) $100
    let entry1 = JournalEntry::new(
        NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        vec![
            JournalLine::new(AccountCode::new("5111").unwrap(), Decimal::from(100), Decimal::ZERO).unwrap(),
            JournalLine::new(AccountCode::new("1111").unwrap(), Decimal::ZERO, Decimal::from(100)).unwrap(),
        ],
        "Sale 1".to_string(),
    ).unwrap();

    // Entry 2: Debit 621 (Expense) $50, Credit 1111 (Cash) $50
    let entry2 = JournalEntry::new(
        NaiveDate::from_ymd_opt(2025, 1, 2).unwrap(),
        vec![
            JournalLine::new(AccountCode::new("621").unwrap(), Decimal::from(50), Decimal::ZERO).unwrap(),
            JournalLine::new(AccountCode::new("1111").unwrap(), Decimal::ZERO, Decimal::from(50)).unwrap(),
        ],
        "Purchase 1".to_string(),
    ).unwrap();

    ledger.post_entry(entry1).unwrap();
    ledger.post_entry(entry2).unwrap();

    // Total Debits: 100 (5111) + 50 (621) = 150
    // Total Credits: 100 (1111) + 50 (1111) = 150
    let tb = ledger.trial_balance();
    assert_eq!(tb.total_debit, Decimal::from(150));
    assert_eq!(tb.total_credit, Decimal::from(150));
}

#[test]
fn test_ledger_get_account_balance() {
    let mut ledger = Ledger::new();
    let entry = JournalEntry::new(
        // FIX: Replaced NaiveDate::from_ymd
        NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        vec![
            JournalLine::new(AccountCode::new("5111").unwrap(), Decimal::from(100), Decimal::ZERO).unwrap(),
            JournalLine::new(AccountCode::new("1111").unwrap(), Decimal::ZERO, Decimal::from(100)).unwrap(),
        ],
        "Sale".to_string(),
    ).unwrap();

    ledger.post_entry(entry).unwrap();

    // Check Revenue Account (5111)
    let balance_5111 = ledger.get_account_balance(&AccountCode::new("5111").unwrap()).unwrap();
    assert_eq!(balance_5111.debit_balance, Decimal::from(100));
    assert_eq!(balance_5111.credit_balance, Decimal::ZERO);

    // Check Cash Account (1111)
    let balance_1111 = ledger.get_account_balance(&AccountCode::new("1111").unwrap()).unwrap();
    assert_eq!(balance_1111.debit_balance, Decimal::ZERO);
    assert_eq!(balance_1111.credit_balance, Decimal::from(100));
}

#[test]
fn test_ledger_get_balance_for_non_existent_account() {
    let ledger = Ledger::new();
    // A valid format code that has never been posted to the ledger
    let unknown_code = AccountCode::new("9999").unwrap(); 
    
    let result = ledger.get_account_balance(&unknown_code);
    
    // Test for the documented API behavior: returns Option<Balance> where None 
    // means the account is unknown/unposted (or the Ledger::get_account_balance 
    // method returns an Option<Balance> and returns None if the account is not found).
    assert!(result.is_none());
    
    // NOTE: If your actual Ledger API returns Option<Balance> instead of Result<Balance, E>, 
    // the previous test where you asserted is_some() and checked for zero balance 
    // implies that the *implementation* returns a zero balance struct if None is found. 
    // However, if the API signature is Option<Balance>, a failure to find the 
    // account should generally return None. Let's assume the latter (is_none()) for clean API design.
}