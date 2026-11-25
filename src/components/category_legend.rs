
use dioxus::prelude::*;

use crate::transactions::analytics::{
    CategorySpending,
    category_color,
};

#[component]
pub fn CategoryLegend(spending: Vec<CategorySpending>, total: i32) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-3 w-full",
            for item in spending.iter() {
                {
                    let percentage = if total > 0 {
                        item.amount as f64 / total as f64 * 100.0
                    } else {
                        0.0
                    };

                    let color = category_color(item.category);
                    rsx! {
                        div {
                            class: "flex items-center justify-between",
                            div {
                                class: "flex items-center gap-3",
                                div {
                                    class: "w-3 h-3 rounded-full",
                                    style: "background-color: {color};"
                                }
                                span {
                                    class: "font-medium text-sm",
                                    "{item.category}"
                                }
                            }

                            div {
                                class: "flex items-center gap-3",
                                span {
                                    class: "font-semibold text-sm",
                                    "{item.amount} MDL"
                                }
                                span {
                                    class: "text-gray-400 text-xs w-12 text-right",
                                    "{percentage:.1}%"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
