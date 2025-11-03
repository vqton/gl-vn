// gl-core/src/period/mod.rs

use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Period {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub status: PeriodStatus,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum PeriodStatus {
    Open,
    Closed,
}

impl Period {
    pub fn new(start: NaiveDate, end: NaiveDate) -> Self {
        Self {
            start_date: start,
            end_date: end,
            status: PeriodStatus::Open,
        }
    }

    pub fn close(&mut self) {
        self.status = PeriodStatus::Closed;
    }

    pub fn is_open(&self) -> bool {
        matches!(self.status, PeriodStatus::Open)
    }

    pub fn contains_date(&self, date: NaiveDate) -> bool {
        date >= self.start_date && date <= self.end_date
    }
}