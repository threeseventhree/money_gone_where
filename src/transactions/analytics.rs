use chrono::Datelike;
use crate::transactions::{
    transaction::Transaction,
    transaction_categories::TransactionCategory,
};

#[derive(Debug, Clone, PartialEq)]
pub struct CategorySpending {
    pub category: TransactionCategory,
    pub amount: i32,
}

pub fn calculate_monthly_spending(
    transactions: &[Transaction],
    month: u32,
    year: i32,
) -> Vec<CategorySpending> {
    let categories = [
        TransactionCategory::Food,
        TransactionCategory::Groceries,
        TransactionCategory::Bills,
        TransactionCategory::Entertainment,
        TransactionCategory::Education,
        TransactionCategory::Coffee,
        TransactionCategory::Cigarettes,
    ];

    categories
        .iter()
        .map(|category| {
            let amount = transactions
                .iter()
                .filter(|transaction| {
                    transaction.category == *category
                        && transaction.date.month() == month
                        && transaction.date.year() == year
                })
                .map(|transaction| transaction.amount)
                .sum();

            CategorySpending {
                category: *category,
                amount,
            }
        })
        .collect()
}

pub fn calculate_total(spending: &[CategorySpending]) -> i32 {
    spending
        .iter()
        .map(|item| item.amount)
        .sum()
}

pub fn category_color(category: TransactionCategory) -> &'static str {
    match category {
        TransactionCategory::Food => "#64748B",
        TransactionCategory::Groceries => "#A1A1AA",
        TransactionCategory::Bills => "#52525B",
        TransactionCategory::Entertainment => "#D4D4D8",
        TransactionCategory::Education => "#94A3B8",
        TransactionCategory::Coffee => "#71717A",
        TransactionCategory::Cigarettes => "#3F3F46",
    }
}