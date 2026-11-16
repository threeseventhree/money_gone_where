#[path = "expense_categories.rs"]
mod expense_category;
use expense_category::*;
use chrono::prelude::*;
use uuid::Uuid;

pub struct Expense {
    pub id: Uuid,
    pub name: String,
    pub amount: i32,
    pub category: ExpenseCategory,
    pub date: NaiveDate,
    pub time: NaiveTime
} impl Expense {
    pub fn new(name: String, amount: i32, category: ExpenseCategory) -> Self {
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