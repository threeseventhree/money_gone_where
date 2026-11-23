use dioxus::prelude::*;
use crate::components::header::Header;

#[component]
pub fn MenuModal(display_menu_modal: Signal<bool>, pages: Vec<String>, current_page: Signal<String>) -> Element {
    rsx! {
        div {
            class: "fixed inset-0 z-50 bg-black/30",
            div {
                class: "w-full h-full bg-white flex flex-col",
                Header {display_menu_modal}
                hr {class: "border-gray-200"}
                for page in pages.iter() {
                    MenuButton {
                        name: page.clone(),
                        onclick: move |selected_page: String| {
                            current_page.set(selected_page);
                            display_menu_modal.set(false);
                        }
                    }
                }
            }
        }
    }
}
#[component]
fn MenuButton(name: String, onclick: EventHandler<String>) -> Element {
    rsx! {
        button {
            class: "p-4 m-2 font-bold text-xl rounded-xl border border-gray-200",
            onclick: move |_| onclick.call(name.clone()),
            "{name}"
        }
    }
}