use std::error::Error;
use uuid::Uuid;

use super::transaction::Transaction;

pub struct TransactionManager {
    pub transactions: Vec<Transaction>,
} impl TransactionManager {
    pub fn new() -> Self {
        Self {
            transactions: Vec::new()
        }
    }
    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), Box<dyn Error>> {
        self.transactions.push(transaction);
        Ok(())
    }
    pub fn edit_transaction(&mut self, id: Uuid, new_name: String, new_amount: i32) -> Result<(), Box<dyn Error>> {
        let transaction = self.transactions.iter_mut().find(|transaction| transaction.id == id);
        match transaction {
            Some(transaction) => {
                transaction.name = new_name;
                transaction.amount = new_amount;
                Ok(())
            },
            None => Err("Transaction not found".into())
        }
    }
}