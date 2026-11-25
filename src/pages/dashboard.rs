use dioxus::prelude::*;

use crate::{components::{footer::Footer, header::Header, recent_transactions_widget::RecentTransactionsWidget, transaction_calendar_widget::TransactionCalendarWidget}, transactions::transaction::Transaction};

#[component]
pub fn DashboardPage(display_menu_modal: Signal<bool>, transactions: Vec<Transaction>, display_transaction_modal: Signal<bool>) -> Element {
    rsx! {
        div {
            class: "flex flex-col min-h-screen w-full",
            Header{
                display_menu_modal
            }
            div {
                class: "flex flex-col flex-1",
                TransactionCalendarWidget {}
                RecentTransactionsWidget{
                    transactions
                }
            }
            Footer {
                display_transaction_modal
            }
        }
    }
}