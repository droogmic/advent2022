// use std::collections::HashMap;

use advent2022_lib::{DayTrait, ParseResult};
use yew::prelude::*;

use crate::file::FileUpload;

pub struct DayBox(pub Box<dyn DayTrait>);

impl PartialEq for DayBox {
    fn eq(&self, other: &Self) -> bool {
        self.0.get_title() == other.0.get_title()
    }
}

#[derive(Properties, PartialEq)]
pub struct DayProps {
    pub day_num: usize,
    pub day: DayBox,
}

#[function_component]
pub fn DayView(props: &DayProps) -> Html {
    let text_format = props.day.0.get_display();
    let part_calculate_func = props.day.0.get_both_func();
    let get_messages = move |input: &str| -> Vec<String> {
        let result: ParseResult<(String, String)> = part_calculate_func(input);
        match result {
            Err(_e) => {
                log::error!("parsing error...");
                vec!["Parsing error, please try again...".to_owned()]
            },
            Ok(answer) => {
                let part1 = format!("Part 1: {}", text_format.0.replace("{answer}", &answer.0));
                let part2 = format!("Part 2: {}", text_format.1.replace("{answer}", &answer.1));
                log::info!("{}", part1);
                log::info!("{}", part2);
                vec![part1, part2]
            },
        }
    };

    let messages = use_state(|| Vec::new());

    let on_run_example = {
        let example = props.day.0.get_example();
        let messages = messages.clone();
        let get_messages = get_messages.clone();
        Callback::from(move |_| {
            log::info!("Running Example");
            messages.set(get_messages(example));
        })
    };

    let on_file_load = {
        let messages_state = messages.clone();
        let get_messages = get_messages.clone();
        Callback::from(move |input: String| {
            log::info!("Running Loaded File");
            let mut messages = get_messages(&input);
            messages.insert(0, String::from("From Upload"));
            messages_state.set(messages);
        })
    };

    let show_input = use_state(|| false);

    let on_collapse = {
        let show_input = show_input.clone();
        Callback::from(move |_| {
            show_input.set(!*show_input);
        })
    };

    let example = props.day.0.get_example();
    html! {
        <section class={if props.day_num & 1 != 0 { "day-odd" } else { "day-even" }}>
            <div class="row">
                <a class="row-item day-key" href={format!("https://adventofcode.com/2022/day/{}", props.day_num)}><h4>{"Day "}{props.day_num}{":"}</h4></a>
                <a class="row-item day-title" href={format!("https://adventofcode.com/2022/day/{}", props.day_num)}><h2><em>{props.day.0.get_title()}</em></h2></a>
                <a class="row-item day-url" href={format!("https://github.com/droogmic/advent2022/blob/main/advent2022-lib/src/day{:02}.rs", props.day_num)}>{"Source Code"}</a>
            </div>
            <div class="row row-reverse">
                <FileUpload day_num={props.day_num} file_load_callback={on_file_load} />
                <div class="row-item day-run">
                    <button type="button" onclick={on_run_example}>{ "▶ Run..." }</button>
                </div>
                <div class="row-item day-collapse">
                    <h5 class={if example.lines().count() > 1 {"button"} else {"button disabled"}} onclick={on_collapse}>
                    {
                        if example.lines().count() > 1 {
                            if *show_input {
                                "▼ Example: "
                            } else {
                                "▬ Example: "
                            }
                        } else {
                            "▬ Example: "
                        }
                    }
                    </h5>
                </div>
            </div>
                {
                    if *show_input {
                        html! {
                            <pre>{example}</pre>
                        }
                    } else {
                        html! {
                            <pre class={if example.lines().count() > 1 {"collapsed"} else {""}}>
                                {example.lines().take(2).collect::<Vec<_>>().join("\n")}
                            </pre>
                        }
                    }
                }
                {
                    for messages.iter().map(|message| {
                        html! {
                            <p>{message}</p>
                        }
                    })
                }
        </section>
    }
}
