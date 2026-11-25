use dioxus::prelude::*;
use std::cmp::Reverse;

use crate::components::transaction_item::TransactionItem;
use crate::transactions::transaction::Transaction;
use crate::transactions::transaction_categories::TransactionCategory;

#[component]
pub fn RecentTransactionsWidget(transactions: Vec<Transaction>) -> Element {
    let mut latest: Vec<&Transaction> = transactions.iter().collect();
    latest.sort_by_key(|t| Reverse((t.date, t.time)));
    latest.truncate(3);
    let empty_slots = 3 - latest.len();
    rsx! {
        div {
            class: "p-4 flex flex-col gap-2 border border-gray-200 rounded-2xl m-4",
            div {
                h1 {
                    class: "text-xl font-bold mb-2",
                    "Recent Transactions"
                }
            }
            div {
                class: "flex flex-col gap-2",
                for transaction in latest {
                    TransactionItem {
                        name: Some(transaction.name.clone()),
                        amount: Some(transaction.amount),
                        category: Some(transaction.category.to_string()),
                        date: Some(transaction.date.format("%d-%m-%y").to_string())
                    }
                }
                for _ in 0..empty_slots {
                    TransactionItem {
                        name: None,
                        amount: None,
                        category: None,
                        date: None
                    }
                }
            }
        }
    }
} 