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
}