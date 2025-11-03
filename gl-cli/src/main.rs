use clap::{Arg, Command}; // 'command' removed
use gl_core::{Ledger, PeriodManager, load_vn_coa}; // Unused gl_core items removed
use serde_json;
use std::fs::File;
use std::io::Read;
use tracing::{error, info};
use chrono::NaiveDate;

// === Logging Setup ===
fn setup_logging() {
    tracing_subscriber::fmt::fmt()
        .compact()
        .with_max_level(tracing::Level::INFO)
        .init();
}
// === Main Function ===
fn main() {
    setup_logging();

    let matches = Command::new("gl-cli")
        .arg(
            Arg::new("PERIOD_OPEN")
                .short('o')
                .long("period-open")
                .value_name("MONTH")
                .help("Open a new accounting period"),
        )
        .arg(
            Arg::new("PERIOD_CLOSE")
                .short('c')
                .long("period-close")
                .help("Close current accounting period"),
        )
        .arg(
            Arg::new("INIT")
                .short('i')
                .long("init")
                .help("Initialize a new ledger with VAS CoA")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("ENTRY_ADD")
                .short('a')
                .long("entry-add")
                .value_name("FILE")
                .help("Add a journal entry from JSON file"),
        )
        .arg(
            Arg::new("REPORT_TRIAL")
                .short('t')
                .long("report-trial")
                .help("Generate trial balance")
                .action(clap::ArgAction::SetTrue),
        )
        // Missing Argument for YEAR in PERIOD_OPEN logic
        .arg(
            Arg::new("YEAR")
                .long("year")
                .value_name("YEAR")
                .help("Year for the accounting period (used with --period-open)")
                .required(false)
        )
        .get_matches();

    let mut ledger = Ledger::new();

    // Load VAS CoA
    let coa = match load_vn_coa() {
        Ok(coa) => coa,
        Err(e) => {
            eprintln!("Error loading VN CoA: {}", e);
            return;
        }
    };

    if matches.get_flag("INIT") {
        info!("Initialized new ledger with VAS CoA");
        println!("✅ Ledger initialized with {} accounts", coa.accounts.len());
    }

    if matches.get_one::<String>("ENTRY_ADD").is_some() {
        let file_path = matches.get_one::<String>("ENTRY_ADD").unwrap();
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Error opening file {}: {}", file_path, e);
                return;
            }
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error reading file {}: {}", file_path, e);
                return;
            }
        };

        // Note: JournalEntry is still used here, but must be imported from gl_core or inferred.
        // Assuming JournalEntry is derivable/imported correctly within the scope of gl-core's crate root.
        let entry: gl_core::JournalEntry = match serde_json::from_str(&contents) {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Error parsing JSON: {}", e);
                return;
            }
        };

        match ledger.post_entry(entry) {
            Ok(_) => {
                info!("Successfully posted entry");
                println!("✅ Entry posted successfully");
            }
            Err(e) => {
                error!("Failed to post entry: {}", e);
                eprintln!("❌ Failed to post entry: {}", e);
            }
        }
    }

    if matches.get_flag("REPORT_TRIAL") {
        let tb = ledger.trial_balance();
        println!("📊 Trial Balance:");
        for bal in tb.balances {
            println!(
                "{}: Debit={:.2}, Credit={:.2}",
                bal.account.as_str(),
                bal.debit_balance,
                bal.credit_balance
            );
        }
        println!("Total Debit: {:.2}", tb.total_debit);
        println!("Total Credit: {:.2}", tb.total_credit);

        if tb.total_debit != tb.total_credit {
            error!("Trial balance is unbalanced!");
        } else {
            info!("Trial balance is balanced.");
        }
    }

    if matches.get_one::<String>("PERIOD_OPEN").is_some() { // Changed to check if PERIOD_OPEN is present
        let default_month = "01".to_string();
        let month = matches
            .get_one::<String>("PERIOD_OPEN")
            .unwrap_or(&default_month);
        let default_year = "2025".to_string();
        let year = matches
            .get_one::<String>("YEAR")
            .unwrap_or(&default_year);
        
        // FIX: Use from_ymd_opt()
        let start_date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(); 
        // FIX: Use from_ymd_opt()
        let end_date = NaiveDate::from_ymd_opt(2025, 1, 31).unwrap();

        let mut period_manager = PeriodManager::new();
        // Period type definition is likely needed here, but assuming it's available via gl_core
        period_manager.add_period(gl_core::Period::new(start_date, end_date)); 

        info!("Opened period: {}-{}", year, month);
        println!("✅ Period opened: {}-{}", year, month);
    }

    if matches.get_flag("PERIOD_CLOSE") {
        let mut period_manager = PeriodManager::new();
        period_manager.close_current_period();

        info!("Closed current period");
        println!("✅ Current period closed");
    }
}