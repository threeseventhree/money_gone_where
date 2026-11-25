use dioxus::prelude::*;
use chrono::{Datelike, Local};

#[component]
pub fn MonthSelector(selected_month: Signal<(u32, i32)>) -> Element {
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let current_year = Local::now().year();
    let years: Vec<i32> = (current_year - 5..=current_year).collect();
    rsx! {
        div {
            class: "flex items-center gap-2",
            select {
                class: "border border-gray-200 rounded-xl p-2 font-medium",
                value: "{selected_month().0}",
                onchange: move |event| {
                    if let Ok(month) = event.value().parse::<u32>() {
                        let (_, year) = selected_month();
                        selected_month.set((month, year));
                    }
                },
                for (index, month) in months.iter().enumerate() {
                    option {
                        value: "{index + 1}",
                        "{month}"
                    }
                }
            }

            select {
                class: "border border-gray-200 rounded-xl p-2 font-medium",
                value: "{selected_month().1}",
                onchange: move |event| {
                    if let Ok(year) = event.value().parse::<i32>() {
                        let (month, _) = selected_month();
                        selected_month.set((month, year));
                    }
                },
                for year in years.iter() {
                    option {
                        value: "{year}",
                        "{year}"
                    }
                }
            }
        }
    }
}