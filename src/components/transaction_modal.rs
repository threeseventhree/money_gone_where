use dioxus::prelude::*;
use dioxus_icons::lucide::{X};

use crate::transactions::transaction_categories::TransactionCategory;

#[component]
pub fn TransactionModal(add_transaction: EventHandler<(String, String, TransactionCategory)>, display_modal: Signal<bool>) -> Element {
    let mut name = use_signal(String::new);
    let mut amount = use_signal(String::new);
    let mut category = use_signal(|| TransactionCategory::Food);
    let create_transaction = move |_| {
        if name().trim().is_empty() {
            return;
        }
        add_transaction.call((
            name().trim().to_string(),
            amount().trim().to_string(),
            category()
        ));
        display_modal.set(false);
    };
    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-black/30",
            div {
                class: "flex flex-col justify-around p-4 bg-white rounded-xl border border-gray-200 gap-4 w-full max-w-md shadow-xl ml-4 mr-4 divide-y divide-gray-200",
                div {
                    class: "flex flex-row justify-between items-center",
                    div {
                        class: "flex flex-col mb-2 ",
                        h1 {
                            class: "font-bold text-xl",
                            "Add transaction"
                        }
                        p {
                            class: "text-md text-gray-500",
                            "Add your new transaction"
                        }
                    }
                    button {
                        class: "p-2 border border-gray-200 rounded-xl hover:bg-gray-100 transition-all duration-300 cursor-pointer",
                        onclick: move |_| {
                            display_modal.set(false);
                        },
                        X {size:20}
                    }
                }
                div {
                    class: "flex flex-col gap-2",
                    div {
                        class: "flex flex-col gap-2",
                        h2 {
                            class: "text-sm text-gray-500",
                            "Transaction Name"
                        }
                        input {
                            class:"px-2 py-1 rounded-xl border border-gray-200 text-sm",
                            r#type: "text",
                            value: "{name}",
                            oninput: move |event| name.set(event.value()),
                            placeholder:"Name...",
                        }
                    }
                    div {
                        class: "flex flex-col gap-2",
                        h2 {
                            class: "text-sm text-gray-500",
                            "Transaction Amount"
                        }
                        input {
                            class:"px-2 py-1 rounded-xl border border-gray-200 text-sm",
                            r#type: "number",
                            value: "{amount}",
                            oninput: move |event| amount.set(event.value()),
                            placeholder:"ex: 50",
                        }
                    }
                    div {
                        class: "flex flex-col gap-2",
                        h2 {
                            class: "text-sm text-gray-500",
                            "Transaction Category"
                        }
                        div {
                            select {
                                class: "border border-gray-200 rounded-xl p-2 w-full",
                                value: "{category:?}",
                                onchange: move |event| {
                                    if let Some(value) = TransactionCategory::from_str(&event.value()) {
                                        category.set(value);
                                    }
                                },
                                option { value: "Food", "Food" }
                                option { value: "Groceries", "Groceries" }
                                option { value: "Bills", "Bills" }
                                option { value: "Entertainment", "Entertainment" }
                                option { value: "Education", "Education" }
                                option { value: "Coffee", "Coffee" }
                                option { value: "Cigarettes", "Cigarettes" }
                            }
                        }
                    }
                    div {
                        class: "w-full",
                        button {
                            class: "rounded-2xl bg-black text-white p-2 font-bold w-full",
                            onclick: create_transaction,
                            "Add Transaction",
                        }
                    }
                }
            }
        }
    }
}