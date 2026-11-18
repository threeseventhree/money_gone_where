use dioxus::prelude::*;
use dioxus_icons::lucide::{BadgeDollarSign, Menu, Plus};

mod components;
use components::transaction_modal::TransactionModal;

mod transactions;
use transactions::transaction::Transaction;
use transactions::transaction_categories::TransactionCategory;
use transactions::transaction_manager::TransactionManager;

mod db;
use db::storage;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut manager: TransactionManager = TransactionManager::new();
    let _ = manager.add_transaction(Transaction::new("Test Transaction".to_string(), 50, TransactionCategory::Food)); 
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div {
            class: "flex flex-col min-h-screen divide-y",
            Header{name: "Dashboard".to_string()}
            div {
                class: "flex flex-1",
                h1{"Hi"}
            }
            Footer {}
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
fn Footer() -> Element {
    rsx! {
        div {
            class: "p-4 flex flex-row justify-between items-center",
            h1 {
                class: "text-xl font-bold mb-1",
                "373's transactions"
            }
            button {
                class: "p-2 rounded-3xl border border-gray-200",
                Plus{}
            }
        }
    }
}
