use yew::prelude::*;

use advent2022_lib::get_days;

mod web;

pub enum Msg {}

pub struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    _link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { _link: link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {}
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let days = get_days();
        html! {
            <div>
                <h1>{"Advent of Code"}</h1>
                {
                    for days.iter().map(|(day_num, day)| {
                        let props = yew::props!(web::Day::Properties {
                            day_num: *day_num,
                            title: day.get_title(),
                            example: day.get_example().to_owned(),
                            both_func: day.get_both_func(),
                            text_format: day.get_display(),
                        });
                        html!{
                            <web::Day with props/>
                        }
                    })
                }
            </div>
        }
    }
}

fn main() {
    console_log::init_with_level(log::Level::Info).expect("logging failed");
    log::trace!("Initializing yew...");
    yew::start_app::<Model>();
}
