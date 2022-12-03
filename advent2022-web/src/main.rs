use advent2022_lib::get_days;
use yew::prelude::*;

use crate::web::{DayBox, DayProps, DayView};

mod file;
mod web;

#[function_component]
fn App() -> Html {
    let mut days = get_days();
    let mut day_nums: Vec<usize> = days.keys().copied().collect();
    day_nums.sort_unstable();
    html! {
        <div>
            <h1>{"Advent of Code"}</h1>
            {
                for day_nums.into_iter().map(|day_num| {
                    let props = yew::props!(DayProps {
                        day_num: day_num,
                        day: DayBox(days.remove(&day_num).unwrap()),
                    });
                    html!{
                        <DayView ..props/>
                    }
                })
            }
        </div>
    }
}

fn main() {
    console_log::init_with_level(log::Level::Info).expect("logging failed");
    log::trace!("Initializing yew...");
    yew::Renderer::<App>::new().render();
}
