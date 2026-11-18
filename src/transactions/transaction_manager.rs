use std::error::Error;
use uuid::Uuid;

use crate::db::storage::Storage;
use super::transaction::Transaction;

pub struct TransactionManager {
    transactions: Vec<Transaction>,
    storage: Storage
} impl TransactionManager {
    pub fn new(storage: Storage) -> Result<Self, Box<dyn Error>> {
        let transactions = storage.load_transactions()?;
        Ok (
            Self {
                transactions,
                storage
            }
        )
    }
    pub fn transactions(&self) -> &[Transaction] {
        &self.transactions
    } // view-only transactions
    fn save(&self) -> Result<(), Box<dyn Error>> {
        self.storage.save_transactions(&self.transactions)
    }
    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), Box<dyn Error>> {
        self.transactions.push(transaction);
        self.save()
    }
    pub fn edit_transaction(&mut self, id: Uuid, new_name: String, new_amount: i32) -> Result<(), Box<dyn Error>> {
        let transaction = self.transactions.iter_mut().find(|transaction| transaction.id == id);
        match transaction {
            Some(transaction) => {
                transaction.name = new_name;
                transaction.amount = new_amount;
            },
            None => return Err("Transaction not found".into())
        }
        self.save()
    }
    pub fn remove_transaction(&mut self, id: Uuid) -> Result<(), Box<dyn Error>> {
        let init_length = self.transactions().len();
        self.transactions.retain(|transaction| transaction.id != id);
        if self.transactions.len() == init_length { return Err("Transaction not found".into()); }
        self.save()
    }
}