use yew::prelude::*;

use advent2022_lib::get_days;

use crate::web::{DayProps, DayView};

mod web;

#[function_component]
fn App() -> Html {
    let days = get_days();
    html! {
        <div>
            <h1>{"Advent of Code"}</h1>
            {
                for days.iter().map(|(day_num, day)| {
                    let props = yew::props!(DayProps {
                        day_num: *day_num,
                        title: day.get_title(),
                        example: day.get_example().to_owned(),
                        text_format: day.get_display(),
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
