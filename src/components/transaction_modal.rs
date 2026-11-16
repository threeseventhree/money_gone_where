use dioxus::prelude::*;

#[component]
pub fn TransactionModal() -> Element {
    rsx! {
        div {
            h1 {
                "Hello Transaction Tracker!"
            }
        }
    }
}