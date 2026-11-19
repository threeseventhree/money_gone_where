use dioxus::prelude::*;

use crate::components::transaction_item::TransactionItem;

#[component]
pub fn RecentTransactionsWidget() -> Element {
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
                TransactionItem {}
                TransactionItem {}
                TransactionItem {}
            }
            
        }
    }
} 