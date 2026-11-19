use dioxus::prelude::*;

#[component]
pub fn TransactionItem() -> Element {
    rsx! {
        div {
            class: "flex flex-row justify-between items-center p-4 rounded-xl border border-gray-200",
            div {
                class: "flex flex-col gap-1",
                h1 {
                    class: "text-xl font-bold",
                    "Sample Transaction"
                }
                p {
                    class: "text-sm text-gray-500",
                    "50 MDL"
                }
            }
            div {
                class: "p-2 border border-gray-200 rounded-2xl",
                h2 {
                    class: "text-lg font-bold",
                    "Food"
                }
                //Icon maybe
            }
        }
    }
}