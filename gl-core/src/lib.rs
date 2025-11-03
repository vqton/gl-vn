use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use thiserror::Error;
use tracing::{info, debug, error};
mod coa;
// === Errors ===
#[derive(Error, Debug)]
pub enum LedgerError {
    #[error("Journal entry is unbalanced: debit ({debit}) != credit ({credit})", debit = .0, credit = .1)]
    UnbalancedEntry(Decimal, Decimal),
    #[error("Invalid account code: {0}")]
    InvalidAccountCode(String),
    #[error("Negative amount not allowed: {0}")]
    NegativeAmount(Decimal),
    #[error("Date must be a valid date")]
    InvalidDate,
}

// === Account Code ===
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AccountCode(pub String);

impl AccountCode {
    pub fn new(code: &str) -> Result<Self, LedgerError> {
        if code.is_empty() || !code.chars().all(|c| c.is_digit(10)) {
            Err(LedgerError::InvalidAccountCode(code.to_string()))
        } else {
            Ok(AccountCode(code.to_string()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// === Journal Line ===
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JournalLine {
    pub account: AccountCode,
    pub debit: Decimal,
    pub credit: Decimal,
}

impl JournalLine {
    pub fn new(account: AccountCode, debit: Decimal, credit: Decimal) -> Result<Self, LedgerError> {
        if debit < Decimal::ZERO {
            return Err(LedgerError::NegativeAmount(debit));
        }
        if credit < Decimal::ZERO {
            return Err(LedgerError::NegativeAmount(credit));
        }
        Ok(JournalLine { account, debit, credit })
    }
}

// === Journal Entry ===
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JournalEntry {
    pub date: NaiveDate,
    pub lines: Vec<JournalLine>,
    pub description: String,
}

impl JournalEntry {
    pub fn new(
        date: NaiveDate,
        lines: Vec<JournalLine>,
        description: String,
    ) -> Result<Self, LedgerError> {
        for line in &lines {
            if line.debit < Decimal::ZERO || line.credit < Decimal::ZERO {
                return Err(LedgerError::NegativeAmount(line.debit.max(line.credit)));
            }
        }

        let total_debit = lines.iter().map(|l| l.debit).sum::<Decimal>();
        let total_credit = lines.iter().map(|l| l.credit).sum::<Decimal>();

        if total_debit != total_credit {
            return Err(LedgerError::UnbalancedEntry(total_debit, total_credit));
        }

        info!("Created balanced journal entry: {:?}", description);
        Ok(JournalEntry { date, lines, description })
    }

    pub fn is_balanced(&self) -> bool {
        let total_debit = self.lines.iter().map(|l| l.debit).sum::<Decimal>();
        let total_credit = self.lines.iter().map(|l| l.credit).sum::<Decimal>();
        total_debit == total_credit
    }
}

// === Ledger ===
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AccountBalance {
    pub account: AccountCode,
    pub debit_balance: Decimal,
    pub credit_balance: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TrialBalance {
    pub balances: Vec<AccountBalance>,
    pub total_debit: Decimal,
    pub total_credit: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ledger {
    balances: Vec<AccountBalance>,
}

impl Ledger {
    pub fn new() -> Self {
        Ledger { balances: vec![] }
    }

    pub fn post_entry(&mut self, entry: JournalEntry) -> Result<(), LedgerError> {
        debug!("Posting entry: {:?}", entry.description);

        for line in entry.lines {
            match self.balances.iter_mut().find(|b| b.account == line.account) {
                Some(balance) => {
                    balance.debit_balance += line.debit;
                    balance.credit_balance += line.credit;
                }
                None => {
                    self.balances.push(AccountBalance {
                        account: line.account.clone(),
                        debit_balance: line.debit,
                        credit_balance: line.credit,
                    });
                }
            }
        }

        info!("Successfully posted entry: {}", entry.description);
        Ok(())
    }

    pub fn trial_balance(&self) -> TrialBalance {
        let total_debit = self.balances.iter().map(|b| b.debit_balance).sum::<Decimal>();
        let total_credit = self.balances.iter().map(|b| b.credit_balance).sum::<Decimal>();

        TrialBalance {
            balances: self.balances.clone(),
            total_debit,
            total_credit,
        }
    }

    pub fn get_account_balance(&self, account_code: &AccountCode) -> Option<&AccountBalance> {
        self.balances.iter().find(|b| b.account == *account_code)
    }
}

// === Helper: Create sample CoA ===
pub fn create_vn_coa() -> Vec<AccountCode> {
    vec![
        AccountCode::new("1111").unwrap(), // Cash
        AccountCode::new("1121").unwrap(), // Bank
        AccountCode::new("131").unwrap(),  // Accounts Receivable
        AccountCode::new("331").unwrap(),  // Accounts Payable
        AccountCode::new("5111").unwrap(), // Revenue
        AccountCode::new("621").unwrap(),  // Cost of Goods Sold
        AccountCode::new("911").unwrap(),  // Profit/Loss
        AccountCode::new("421").unwrap(),  // Retained Earnings
    ]
}

pub use coa::{Account, CoaConfig, load_vn_coa};