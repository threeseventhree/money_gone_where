use dioxus::prelude::*;
use dioxus_icons::lucide::{BadgeDollarSign, Menu, Plus};

mod components;
use components::transaction_modal::TransactionModal;

mod db;
use db::storage::Storage;

mod transactions;
use transactions::transaction_manager::TransactionManager;

use crate::transactions::{transaction::Transaction, transaction_categories::TransactionCategory};

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let storage = Storage::new("data/transactions.json");
    let mut manager = use_signal(|| {TransactionManager::new(storage).expect("Failed to load transactions")});
    let display_modal = use_signal(|| false);

    let mut add_transaction = move |(name, amount, category): (String, String, TransactionCategory)| {
        let Ok(amount) = amount.parse::<i32>() else {
            return;
        };

        let transaction = Transaction::new(name, amount, category);
        if let Err(error) = manager.write().add_transaction(transaction) {
            eprintln!("Failed to add transaction: {error}");
        }
    };

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div {
            class: "flex flex-col min-h-screen divide-y",
            Header{name: "Dashboard".to_string()}
            div {
                class: "flex flex-1",
                for transaction in manager.read().transactions().iter() {
                    SampleTransactionDisplay {
                        name: transaction.name.clone(),
                        amount: transaction.amount.to_string(),
                        category: format!("{:?}", transaction.category)
                    }
                }
            }
            Footer {
                display_modal
            }
        }
        if display_modal() {
            TransactionModal {
                add_transaction: move |data| add_transaction(data),
                display_modal
            }
        }
    }
}

#[component]
fn Header(name: String) -> Element {
    rsx! {
        div {
            class: "p-4 flex flex-row justify-between items-center ",
            div {
                class: "flex flex-row gap-2 items-center",
                BadgeDollarSign {size: 24}
                h1 {
                    class: "font-bold text-xl mb-1",
                    "{name}"
                }
            }
            button {
                class: "p-2 rounded-3xl border border-gray-200",
                Menu{}
            }
        }
    }
}
#[component]
fn Footer(display_modal: Signal<bool>) -> Element {
    rsx! {
        div {
            class: "p-4 flex flex-row justify-between items-center",
            h1 {
                class: "text-xl font-bold mb-1",
                "373's transactions"
            }
            button {
                class: "p-2 rounded-3xl border border-gray-200",
                onclick: move |_| {
                    display_modal.set(true);
                },
                Plus{}
            }
        }
    }
}

#[component]
fn SampleTransactionDisplay(name: String, amount: String, category: String) -> Element {
    rsx! {
        div {
            h1 {
                "{name}"
            }
            h1 {
                "{amount}"
            }
            h1 {
                "{category}"
            }
        }
    }
}