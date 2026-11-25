
use dioxus::prelude::*;
use chrono::{Datelike, Local};

use crate::{
    components::{
        footer::Footer,
        header::Header,
        month_selector::MonthSelector,
        donut_chart::DonutChart,
        category_legend::CategoryLegend,
    },
    transactions::{
        transaction::Transaction,
        analytics::{
            calculate_monthly_spending,
            calculate_total,
        },
    },
};

#[component]
pub fn SummariesPage(display_menu_modal: Signal<bool>, display_transaction_modal: Signal<bool>, transactions: Vec<Transaction>) -> Element {
    let now = Local::now();
    let selected_month = use_signal(|| {(now.month(), now.year())});
    let (month, year) = selected_month();
    let spending = calculate_monthly_spending(
        &transactions,
        month,
        year,
    );
    let total = calculate_total(&spending);
    rsx! {
        div {
            class: "flex flex-col min-h-screen",
            Header {
                display_menu_modal
            }
            main {
                class: "flex flex-col flex-1 p-4 gap-4",
                div {
                    class: "flex items-center justify-between",
                    h1 {
                        class: "font-bold text-2xl",
                        "Spending Summary"
                    }
                    MonthSelector {
                        selected_month
                    }
                }
                div {
                    class: "flex flex-col gap-4 p-4 rounded-xl border border-gray-200",
                    DonutChart {
                        spending: spending.clone(),
                        total,
                    }
                    div {
                        class: "border-t border-gray-200 pt-4",
                        h2 {
                            class: "font-bold text-lg mb-4",
                            "Categories"
                        }
                        CategoryLegend {
                            spending: spending.clone(),
                            total,
                        }
                    }
                }
                div {
                    class: "flex flex-col gap-1 p-4 rounded-xl border border-gray-200",
                    span {
                        class: "text-sm text-gray-500",
                        "Total spending"
                    }
                    h2 {
                        class: "font-bold text-2xl",
                        "{total} MDL"
                    }
                }
            }
            Footer {
                display_transaction_modal
            }
        }
    }
}
