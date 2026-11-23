use dioxus::prelude::*;

mod components;
use components::transaction_modal::TransactionModal;
use components::recent_transactions_widget::RecentTransactionsWidget;
use components::transaction_calendar_widget::TransactionCalendarWiddget;
use components::header::Header;
use components::footer::Footer;
use components::menu_modal::MenuModal;

mod db;
use db::storage::Storage;

mod transactions;
use transactions::transaction_manager::TransactionManager;
use transactions::transaction::Transaction;
use transactions::transaction_categories::TransactionCategory;

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
        div {
            class: "flex flex-col min-h-screen w-full divide-y divide-gray-200",
            Header{
                display_menu_modal
            }
            div {
                class: "flex flex-col flex-1",
                TransactionCalendarWiddget {}
                RecentTransactionsWidget{
                    transactions
                }
            }
            Footer {
                display_transaction_modal
            }
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

