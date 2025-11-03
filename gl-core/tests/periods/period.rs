use super::*;

#[test]
fn test_period_new() {
    let start = NaiveDate::from_ymd(2025, 1, 1);
    let end = NaiveDate::from_ymd(2025, 1, 31);
    let period = Period::new(start, end);
    assert_eq!(period.start_date, start);
    assert_eq!(period.end_date, end);
    assert_eq!(period.status, PeriodStatus::Open);
}

#[test]
fn test_period_close() {
    let mut period = Period::new(
        NaiveDate::from_ymd(2025, 1, 1),
        NaiveDate::from_ymd(2025, 1, 31),
    );
    period.close();
    assert_eq!(period.status, PeriodStatus::Closed);
}

#[test]
fn test_period_is_open() {
    let period = Period::new(
        NaiveDate::from_ymd(2025, 1, 1),
        NaiveDate::from_ymd(2025, 1, 31),
    );
    assert!(period.is_open());

    let mut period = period;
    period.close();
    assert!(!period.is_open());
}

#[test]
fn test_period_contains_date() {
    let period = Period::new(
        NaiveDate::from_ymd(2025, 1, 1),
        NaiveDate::from_ymd(2025, 1, 31),
    );

    assert!(period.contains_date(NaiveDate::from_ymd(2025, 1, 1)));
    assert!(period.contains_date(NaiveDate::from_ymd(2025, 1, 15)));
    assert!(period.contains_date(NaiveDate::from_ymd(2025, 1, 31)));
    assert!(!period.contains_date(NaiveDate::from_ymd(2024, 12, 31)));
    assert!(!period.contains_date(NaiveDate::from_ymd(2025, 2, 1)));
}

#[test]
fn test_period_manager_add_period() {
    let mut manager = PeriodManager::new();
    let period = Period::new(
        NaiveDate::from_ymd(2025, 1, 1),
        NaiveDate::from_ymd(2025, 1, 31),
    );
    manager.add_period(period);
    assert_eq!(manager.periods.len(), 1);
}

#[test]
fn test_period_manager_get_current_period() {
    let mut manager = PeriodManager::new();
    let period1 = Period::new(
        NaiveDate::from_ymd(2025, 1, 1),
        NaiveDate::from_ymd(2025, 1, 31),
    );
    let period2 = Period::new(
        NaiveDate::from_ymd(2025, 2, 1),
        NaiveDate::from_ymd(2025, 2, 28),
    );

    manager.add_period(period1);
    manager.add_period(period2);

    let current = manager.get_current_period().unwrap();
    assert_eq!(current.end_date, NaiveDate::from_ymd(2025, 2, 28));
}

#[test]
fn test_period_manager_close_current_period() {
    let mut manager = PeriodManager::new();
    let period = Period::new(
        NaiveDate::from_ymd(2025, 1, 1),
        NaiveDate::from_ymd(2025, 1, 31),
    );
    manager.add_period(period);
    manager.close_current_period();

    let current = manager.get_current_period().unwrap();
    assert_eq!(current.status, PeriodStatus::Closed);
}

#[test]
fn test_period_manager_is_date_in_open_period() {
    let mut manager = PeriodManager::new();
    let period = Period::new(
        NaiveDate::from_ymd(2025, 1, 1),
        NaiveDate::from_ymd(2025, 1, 31),
    );
    manager.add_period(period);

    assert!(manager.is_date_in_open_period(NaiveDate::from_ymd(2025, 1, 15)));
    assert!(!manager.is_date_in_open_period(NaiveDate::from_ymd(2025, 2, 1)));

    // Close period
    manager.close_current_period();
    assert!(!manager.is_date_in_open_period(NaiveDate::from_ymd(2025, 1, 15)));
}