// gl-core/src/coa/mod.rs

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub code: String,
    pub name: String,
    pub classification: String,
    pub default_balance: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoaConfig {
    pub accounts: Vec<Account>,
}

pub fn load_vn_coa() -> Result<CoaConfig, Box<dyn std::error::Error>> {
    let mut file = File::open("data/vn_coa.yaml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let coa: CoaConfig = serde_yaml::from_str(&contents)?;
    Ok(coa)
}