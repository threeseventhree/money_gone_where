use dioxus::prelude::*;
use chrono::{NaiveDate, Duration};
use dioxus_icons::lucide::{ArrowLeft, ArrowRight};

use crate::components::{header::Header, transaction_item::TransactionItem, footer::Footer};
use crate::transactions::{transaction::Transaction, transaction_categories::TransactionCategory};

#[component]
pub fn TransactionListPage(display_menu_modal: Signal<bool>, display_transaction_modal: Signal<bool>, transactions: Vec<Transaction>) -> Element {
    rsx! {
        div {
            class: "flex flex-col min-h-screen",
            Header{
                display_menu_modal
            }
            TransactionList {
                transactions
            }
            Footer {
                display_transaction_modal
            }
        }
    }
}
#[component]
fn TransactionList(transactions: Vec<Transaction>) -> Element {
    let selected_date = use_signal(|| chrono::Local::now().date_naive());
    let filtered_transactions: Vec<Transaction> = transactions
    .iter()
    .filter(|transaction| transaction.date == selected_date())
    .cloned()
    .collect();

    rsx! {
        div {
            class: "m-2 p-4 rounded-xl border border-gray-200 flex flex-col flex-1 min-h-0 gap-4",
            div {
                class: "flex flex-col gap-2 ",
                h1 {
                    class: "font-bold text-xl",
                    "Transaction List"
                }
                DateNavigator {
                    selected_date
                }
            }
            hr {class: "border-gray-200"}
            if filtered_transactions.is_empty() {
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

                    for transaction in filtered_transactions.iter() {
                        TransactionItem {
                            name: Some(transaction.name.clone()),
                            amount: Some(transaction.amount),
                            category: Some(TransactionCategory::as_string(transaction.category)),
                            date: Some(transaction.date.to_string())
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn DateNavigator(mut selected_date: Signal<NaiveDate>) -> Element {
    let date = selected_date().format("%d %B %Y");
    rsx! {
        div {
            class: "flex flex-row justify-between items-center",

            h2 {
                class: "text-gray-500 text-md",
                "{date}"
            }

            div {
                class: "flex flex-row gap-2",

                button {
                    class: "p-2 rounded-3xl border border-gray-200",
                    onclick: move |_| {
                        selected_date.set(selected_date() - Duration::days(1));
                    },
                    ArrowLeft { size: 20 }
                }

                button {
                    class: "p-2 rounded-3xl border border-gray-200",
                    onclick: move |_| {
                        selected_date.set(selected_date() + Duration::days(1));
                    },
                    ArrowRight { size: 20 }
                }
            }
        }
    }
}