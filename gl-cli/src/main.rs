use gl_core::{Ledger, JournalEntry, AccountCode, create_vn_coa, load_vn_coa};
use serde_json;
use tracing::{error, info};
use std::fs::File;
use std::io::Read;
use clap::{command, Arg, Command};

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
        .arg(Arg::new("INIT")
            .short('i')
            .long("init")
            .help("Initialize a new ledger with VAS CoA")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("ENTRY_ADD")
            .short('a')
            .long("entry-add")
            .value_name("FILE")
            .help("Add a journal entry from JSON file"))
        .arg(Arg::new("REPORT_TRIAL")
            .short('t')
            .long("report-trial")
            .help("Generate trial balance")
            .action(clap::ArgAction::SetTrue))
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

        let entry: JournalEntry = match serde_json::from_str(&contents) {
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
            println!("{}: Debit={:.2}, Credit={:.2}", bal.account.as_str(), bal.debit_balance, bal.credit_balance);
        }
        println!("Total Debit: {:.2}", tb.total_debit);
        println!("Total Credit: {:.2}", tb.total_credit);

        if tb.total_debit != tb.total_credit {
            error!("Trial balance is unbalanced!");
        } else {
            info!("Trial balance is balanced.");
        }
    }
}