use dioxus::prelude::*;
use chrono::{Datelike, Local, Duration};

#[component]
pub fn TransactionCalendarWidget() -> Element {
    let today = Local::now().date_naive();
    let year = today.year();
    let month = today.month();
    let month_names = [
        "January", "February", "March", "April",
        "May", "June", "July", "August",
        "September", "October", "November", "December"
    ];
    let month_name = month_names[(month - 1) as usize];
    let first_day = today.with_day(1).unwrap();
    let next_month = first_day
        .checked_add_months(chrono::Months::new(1))
        .unwrap();
    let days_in_month = (next_month - Duration::days(1)).day();
    let starting_weekday = first_day
        .weekday()
        .num_days_from_monday();
    let mut calendar_days: Vec<Option<u32>> =
        vec![None; starting_weekday as usize];
    for day in 1..=days_in_month {
        calendar_days.push(Some(day));
    }
    rsx! {
        div {
            class: "p-4 flex flex-col gap-2 border border-gray-200  rounded-2xl m-4 mb-0 mt-2 h-full",
            div {
                class: "flex flex-col",

                h1 {
                    class: "text-xl font-bold",
                    "Transactions Calendar"
                }

                div {
                    class: "text-sm font-semibold text-gray-500 mb-1",
                    "{month_name} {year}"
                }
            }

            div {
                class: "grid grid-cols-7 gap-y-2 text-center",
                for weekday in ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"] {
                    div {
                        class: "text-xs font-semibold text-gray-400",
                        "{weekday}"
                    }
                }

                for day in calendar_days {
                    div {
                        class: "flex justify-center items-center",
                        if let Some(day) = day {
                            div {
                                class: if day == today.day() {
                                    "w-8 h-8 rounded-full flex items-center justify-center bg-black text-white font-bold text-sm"
                                } else {
                                    "w-8 h-8 rounded-full flex items-center justify-center text-sm text-gray-700 hover:bg-gray-100"
                                },

                                "{day}"
                            }
                        }
                    }
                }
            }
        }
    }
}