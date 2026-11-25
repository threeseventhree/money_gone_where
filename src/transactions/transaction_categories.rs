use std::fmt::Display;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TransactionCategory {
    Food,
    Groceries,
    Bills,
    Entertainment,
    Education,
    Coffee,
    Cigarettes
} impl TransactionCategory {
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "Food" => Some(Self::Food),
            "Groceries" => Some(Self::Groceries),
            "Bills" => Some(Self::Bills),
            "Entertainment" => Some(Self::Entertainment),
            "Education" => Some(Self::Education),
            "Coffee" => Some(Self::Coffee),
            "Cigarettes" => Some(Self::Cigarettes),
            _ => None,
        }
    }
} impl Display for TransactionCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionCategory::Food => write!(f, "Food"),
            TransactionCategory::Groceries => write!(f, "Groceries"),
            TransactionCategory::Bills => write!(f, "Bills"),
            TransactionCategory::Entertainment => write!(f, "Entertainment"),
            TransactionCategory::Education => write!(f, "Education"),
            TransactionCategory::Coffee => write!(f, "Coffee"),
            TransactionCategory::Cigarettes => write!(f, "Cigarettes"),
        }
        
    }
}