use dioxus::prelude::*;
use dioxus_icons::lucide::Plus;

#[component]
pub fn Footer(display_transaction_modal: Signal<bool>) -> Element {
    rsx! {
        div {
            class: "p-4 flex flex-row justify-between items-center",
            h1 {
                class: "text-xl font-bold mb-1",
                "373's transactions"
            }
            button {
                class: "p-2 rounded-3xl border border-gray-200",
                onclick: move |_| {
                    display_transaction_modal.set(true);
                },
                Plus{}
            }
        }
    }
}