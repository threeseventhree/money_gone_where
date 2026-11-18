use std::{error::Error, fs, path::PathBuf};

use crate::transactions::transaction::Transaction;

pub struct Storage {
    transactions_path: PathBuf,
} impl Storage {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        println!("Working directory: {:?}", std::env::current_dir());
        println!("Storage file path: {:?}", path);
        Self { transactions_path: path }
    }
    pub fn load_transactions(&self) -> Result<Vec<Transaction>, Box<dyn Error>> {
        if !self.transactions_path.exists() {
            return Ok(Vec::new())
        }

        let contents = fs::read_to_string(&self.transactions_path)?;
        let transactions: Vec<Transaction> = serde_json::from_str(&contents)?;
        Ok(transactions)
    }
    pub fn save_transactions(&self, transactions: &[Transaction]) -> Result<(), Box<dyn Error>> {
        if let Some(parent) = self.transactions_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(transactions)?;
        fs::write(&self.transactions_path, json)?;
        println!("Successfully saved to: {:?}", self.transactions_path);
        println!("Saved {} transactions", transactions.len());
        Ok(())
    }
}
