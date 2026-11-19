use dioxus::prelude::*;

#[component]
pub fn TransactionItem(name: Option<String>, amount: Option<i32>, category: Option<String>, date: Option<String>) -> Element {
    rsx! {
        div {
            class: "flex flex-row justify-between items-center p-4 rounded-xl border border-gray-200 min-h-[80px]",
            div {
                class: "flex flex-col gap-1",
                h1 {
                    class: "text-xl font-bold",

                    if let Some(name) = name {
                        "{name}"
                    } else {
                        span {
                            class: "text-gray-400",
                            "No transaction"
                        }
                    }
                }
                if let (Some(date), Some(amount)) = (date, amount) {
                    div {
                        class: "flex flex-row gap-1",
                        p {
                            class: "text-sm text-gray-500",
                            "{date}"
                        }

                        p {
                            class: "text-sm text-gray-500",
                            "• {amount} MDL"
                        }
                    }
                }
            }
            if let Some(category) = category {
                div {
                    class: "p-2 border border-gray-200 rounded-2xl",
                    h2 {
                        class: "text-sm font-bold",
                        "{category}"
                    }
                }
            }
        }
    }
}