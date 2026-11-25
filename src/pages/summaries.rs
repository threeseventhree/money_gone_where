use dioxus::prelude::*;

use crate::{components::{footer::Footer, header::Header, transaction_item::TransactionItem}, transactions::transaction::{Transaction}};

#[component]
pub fn SummariesPage(display_menu_modal: Signal<bool>, display_transaction_modal: Signal<bool>, transactions: Vec<Transaction>) -> Element {
    let category = use_signal(|| "Food".to_string());
    let filtered_transactions: Vec<Transaction> = transactions
    .iter()
    .filter(|transaction| transaction.category.to_string() == category().to_string())
    .cloned()
    .collect();
    rsx! {
        div {
            class: "flex flex-col min-h-screen",
            Header{
                display_menu_modal
            }
            SummaryDisplay {
                transactions: filtered_transactions,
                category: category
            }
            Footer {
                display_transaction_modal
            }
        }
    }
}

#[component]
fn SummaryDisplay(
    transactions: Vec<Transaction>,
    category: Signal<String>
) -> Element {
    let total_amount: i32 = transactions
        .iter()
        .map(|transaction| transaction.amount)
        .sum();

    rsx! {
        div {
            class: "m-2 p-4 rounded-xl border border-gray-200 flex flex-1 flex-col gap-2",
            div {
                class: "flex flex-col gap-1",
                div {
                    class: "flex flex-row justify-between items-center",
                    h1 {
                        class: "font-bold text-xl",
                        "Transactions for:"
                    }

                    select {
                        class: "border border-gray-200 rounded-xl p-1 font-bold",
                        value: "{category}",
                        onchange: move |event| category.set(event.value()),

                        option { value: "Food", "Food" },
                        option { value: "Groceries", "Groceries" },
                        option { value: "Bills", "Bills" },
                        option { value: "Entertainment", "Entertainment" },
                        option { value: "Education", "Education" },
                        option { value: "Coffee", "Coffee" },
                        option { value: "Cigarettes", "Cigarettes" },
                    }
                }

                h2 {
                    class: "text-md text-gray-500",
                    "Total amount spent on this category is: {total_amount} MDL"
                }
            }
            div {
                if transactions.is_empty() {
                    div {
                        class: "flex-1 flex items-center justify-center",
                        p {
                            class: "text-gray-400 font-medium text-lg",
                            "No transactions yet"
                        }
                    }
                } else {
                    div {
                        class: "flex flex-col flex-1 min-h-0 gap-2 overflow-y-auto",
                        for transaction in transactions.iter() {
                            TransactionItem {
                                name: Some(transaction.name.clone()),
                                amount: Some(transaction.amount),
                                category: Some(transaction.category.to_string()),
                                date: Some(transaction.date.format("%d-%m-%y").to_string())
                            }
                        }
                    }
                }
            }
        }
    }
}