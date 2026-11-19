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
    pub fn as_string(value: TransactionCategory) -> String {
        match value {
            TransactionCategory::Food => "Food".to_string(),
            TransactionCategory::Groceries => "Groceries".to_string(),
            TransactionCategory::Bills => "Bills".to_string(),
            TransactionCategory::Entertainment => "Entertainment".to_string(),
            TransactionCategory::Education => "Education".to_string(),
            TransactionCategory::Coffee => "Coffee".to_string(),
            TransactionCategory::Cigarettes => "Cigarettes".to_string(),
        }
    }
}