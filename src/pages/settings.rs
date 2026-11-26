use dioxus::prelude::*;
use dioxus_icons::lucide::{Settings};
use crate::components::header::Header;

#[component]
pub fn SettingsPage(display_menu_modal: Signal<bool>) -> Element {
    rsx! {
        div {
            class: "flex flex-col min-h-screen",
            Header {
                display_menu_modal
            }
            div {
                class: "m-2 p-4 rounded-xl border border-gray-200 flex flex-col gap-4",
                div {
                    class: "flex flex-row gap-2 items-center",
                    Settings{size: 20}
                    h1 {
                        class: "font-bold text-xl mb-0.5",
                        "Settings"
                    }
                }
                
                div {
                    class: "flex flex-col gap-1",
                    h2 {
                        class: "font-bold text-lg",
                        "Theme: Light/Dark"
                    }
                    div {
                        class: "flex flex-row justify-between gap-1",
                        button {
                            class: "w-1/2 rounded-xl p-3 border border-gray-200 font-bold",
                            "Light"

                        }
                        button {
                            class: "w-1/2 rounded-xl p-3 border border-gray-200 font-bold",
                            "Dark"
                        }
                    }
                }
                div {
                    class: "flex flex-col gap-1",
                    h2 {
                        class: "font-bold text-lg",
                        "Daily Transaction Threshold"
                    }
                    input {
                        class: "p-3 border border-gray-200 rounded-xl outline-none focus:border-gray-400 transition-colors",
                        min: "0",
                        r#type: "number",
                        placeholder: "ex. 1000"
                    }
                }
                div {
                    class: "flex flex-col gap-1",
                    h2 {
                        class: "font-bold text-lg",
                        "Currency"
                    }
                    select {
                        class: "p-3 border border-gray-200 rounded-xl",
                        value: "MDL",
                        option{value: "MDL", "MDL"},
                        option{value: "USD", "USD"},
                        option{value: "EUR", "EUR"},
                    }
                }
                button {
                    class: "p-2 border border-gray-200 rounded-xl font-bold",
                    "Save Changes"
                }
            }
        }
        
    }
}