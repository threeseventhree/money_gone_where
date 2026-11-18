use super::transaction_categories::TransactionCategory;
use chrono::prelude::*;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub name: String,
    pub amount: i32,
    pub category: TransactionCategory,
    pub date: NaiveDate,
    pub time: NaiveTime
} impl Transaction {
    pub fn new(name: String, amount: i32, category: TransactionCategory) -> Self {
        let now: DateTime<Local> = Local::now();
        Self {
            id: Uuid::new_v4(),
            name,
            amount,
            category: category,
            date: now.date_naive(),
            time: now.time()
        }
    }
}