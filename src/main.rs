use dioxus::prelude::*;


mod components;
use components::transaction_modal::TransactionModal;
use components::menu_modal::MenuModal;

mod db;
use db::storage::Storage;

mod transactions;
use transactions::transaction_manager::TransactionManager;
use transactions::transaction::Transaction;
use transactions::transaction_categories::TransactionCategory;

mod pages;
use pages::dashboard::DashboardPage;
use pages::transactionlist::TransactionListPage;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let pages = vec!["Dashboard".to_string(), "Transaction List".to_string(), "Summaries".to_string(), "Settings".to_string()];

    let storage = Storage::new("data/transactions.json");
    let mut manager = use_signal(|| {TransactionManager::new(storage).expect("Failed to load transactions")});
    let display_transaction_modal = use_signal(|| false);
    let display_menu_modal = use_signal(|| false);
    let current_page = use_signal(|| "Dashboard".to_string());

    let transactions = manager.read().transactions().to_vec();

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
        DisplayPages {
            current_page,
            display_menu_modal,
            transactions,
            display_transaction_modal
        }
        if display_transaction_modal() {
            TransactionModal {
                add_transaction: move |data| add_transaction(data),
                display_transaction_modal
            }
        }
        if display_menu_modal() {
            MenuModal {
                display_menu_modal,
                pages,
                current_page
            }
        }
    }
}

#[component]
fn DisplayPages(current_page: Signal<String>, display_menu_modal: Signal<bool>, transactions: Vec<Transaction>, display_transaction_modal: Signal<bool>) -> Element {
    match current_page().as_str() {
    "Dashboard" => rsx! {
        DashboardPage {display_menu_modal, transactions, display_transaction_modal}
    },
    "Transaction List" => rsx! {
        TransactionListPage {
            display_menu_modal, 
            transactions, 
            display_transaction_modal
        }
    },
    "Summaries" => rsx! {
        div { "Summaries" }
    },
    "Settings" => rsx! {
        div { "Settings" }
    },
    _ => rsx! {
        div { "Page not found" }
    }
}
}

