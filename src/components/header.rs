use dioxus::prelude::*;
use dioxus_icons::lucide::{BadgeDollarSign, Menu};

#[component]
pub fn Header(display_menu_modal: Signal<bool>) -> Element {
    rsx! {
        div {
            class: "p-4 flex flex-row justify-between items-center ",
            div {
                class: "flex flex-row gap-2 items-center",
                BadgeDollarSign {size: 24}
                h1 {
                    class: "font-bold text-xl mb-1",
                    "Tracker"
                }
            }
            button {
                class: "p-2 rounded-3xl border border-gray-200",
                onclick: move |_| {
                    display_menu_modal.toggle();
                },
                Menu{}
            }
        }
    }
}