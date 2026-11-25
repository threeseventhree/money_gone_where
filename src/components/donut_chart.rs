use dioxus::prelude::*;

use crate::transactions::analytics::{
    CategorySpending,
    category_color,
};

const RADIUS: f64 = 76.0;
const CIRCUMFERENCE: f64 = 2.0 * std::f64::consts::PI * RADIUS;

#[component]
pub fn DonutChart(spending: Vec<CategorySpending>, total: i32) -> Element {
    let mut offset = 0.0;
    let segments: Vec<_> = spending
        .iter()
        .filter(|item| item.amount > 0)
        .map(|item| {
            let length = item.amount as f64
                / total as f64
                * CIRCUMFERENCE;

            let segment = (
                category_color(item.category),
                length,
                offset,
            );

            offset += length;

            segment
        })
        .collect();
    rsx! {
        div {
            class: "flex items-center justify-center py-4",
            svg {
                view_box: "0 0 200 200",
                class: "w-64 h-64",
                circle {
                    cx: "100",
                    cy: "100",
                    r: "{RADIUS}",
                    fill: "none",
                    stroke: "#E5E7EB",
                    stroke_width: "20",
                }
                for (color, length, offset) in segments.iter() {
                    circle {
                        cx: "100",
                        cy: "100",
                        r: "{RADIUS}",
                        fill: "none",
                        stroke: "{color}",
                        stroke_width: "20",
                        stroke_dasharray: "{length} {CIRCUMFERENCE}",
                        stroke_dashoffset: "-{offset}",
                        transform: "rotate(-90 100 100)",
                    }
                }
                text {
                    x: "100",
                    y: "91",
                    text_anchor: "middle",
                    font_size: "11",
                    fill: "currentColor",
                    "TOTAL SPENT"
                }
                text {
                    x: "100",
                    y: "117",
                    text_anchor: "middle",
                    font_size: "21",
                    font_weight: "bold",
                    fill: "currentColor",
                    "{total}"
                }
                text {
                    x: "100",
                    y: "134",
                    text_anchor: "middle",
                    font_size: "11",
                    fill: "currentColor",
                    "MDL"
                }
            }
        }
    }
}