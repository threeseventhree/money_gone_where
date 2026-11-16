use dioxus::prelude::*;

mod components;
use components::transaction_modal::TransactionModal;

mod transactions;
use transactions::transaction::Transaction;
use transactions::transaction_categories::TransactionCategory;
use transactions::transaction_manager::TransactionManager;

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
        TransactionModal {}
    }
}
