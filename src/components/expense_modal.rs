use dioxus::prelude::*;

#[component]
pub fn ExpenseModal() -> Element {
    rsx! {
        div {
            h1 {
                "Hello Expense Tracker!"
            }
        }
    }
}