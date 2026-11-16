use dioxus::prelude::*;

#[path = "components/expense_modal.rs"]
mod expense_modal;
use expense_modal::*;

#[path = "expenses/expense.rs"]
mod expense;
use expense::Expense;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        ExpenseModal {}
    }
}
