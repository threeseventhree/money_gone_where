use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransactionCategory {
    Food,
    Groceries,
    Bills,
    Entertainment,
    Educations,
    Coffee,
}