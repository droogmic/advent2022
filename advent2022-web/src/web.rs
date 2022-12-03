// use std::collections::HashMap;

use advent2022_lib::ParseError;
// use gloo_file::callbacks::FileReader;
// use gloo_file::File;
use yew::prelude::*;
use yew::Properties;

use advent2022_lib::ParseResult;

// impl PartialEq for dyn Fn(&str) -> ParseResult<(String, String)> {
//     fn eq(&self, other: &Self) -> bool {
//         todo!()
//     }
// }

#[derive(Properties, PartialEq)]
pub struct DayProps {
    pub day_num: usize,
    pub title: &'static str,
    pub example: String,
    pub text_format: (&'static str, &'static str),
    // pub both_func: Rc<dyn Fn(&str) -> ParseResult<(String, String)>>,
}

#[function_component]
pub fn DayView(props: &DayProps) -> Html {
    // let file_upload_id = format!("file-upload-day-{}", props.day_num);
    // let on_file_upload: Callback<String> = Callback::from(move |e: Event| {
    //     let input: HtmlInputElement = e.target_unchecked_into();
    //     let mut result = Vec::new();
    //     if let Some(files) = input.files() {
    //         let files = js_sys::try_iter(&files)
    //             .unwrap()
    //             .unwrap()
    //             .map(|v| web_sys::File::from(v.unwrap()))
    //             .map(File::from);
    //         result.extend(files);
    //     }
    // });
    let messages = use_state(|| Vec::new());
    let on_run_example = {
        let messages = messages.clone();
        let text_format = props.text_format;
        Callback::from(move |_| {
            log::info!("Running Example");
            let result: ParseResult<(String, String)> = Err(ParseError::Empty);
            match result {
                Err(_e) => {
                    log::error!("parsing error...");
                    messages.set(vec!["Parsing error, please try again...".to_owned()])
                }
                Ok(answer) => {
                    let part1 = format!("Part 1: {}", text_format.0.replace("{answer}", &answer.0));
                    let part2 = format!("Part 2: {}", text_format.1.replace("{answer}", &answer.1));
                    log::info!("{}", part1);
                    log::info!("{}", part2);
                    messages.set(vec![part1, part2])
                }
            }
        })
    };
    let show_input = use_state(|| false);
    let on_collapse = {
        let show_input = show_input.clone();
        Callback::from(move |_| {
            show_input.set(!*show_input);
        })
    };
    html! {
        <section class={if props.day_num & 1 != 0 { "day-odd" } else { "day-even" }}>
            <div class="row">
                <div class="row-item day-key"><h4>{"Day "}{props.day_num}{":"}</h4></div>
                <div class="row-item day-title"><h2><em>{&props.title}</em></h2></div>
                <div class="row-item day-url"><a href={format!("https://github.com/droogmic/advent2022/blob/master/advent2022-lib/src/day{:02}.rs", props.day_num)}>{"Source Code"}</a></div>
            </div>
            <div class="row row-reverse">
                // <div class="row-item day-file">
                //     <label for={file_upload_id.clone()} class="custom-file-upload">{"📄 Upload..."}</label>
                //     <input id={file_upload_id.clone()} type="file" onchange=... />
                // </div>
                <div class="row-item day-run">
                    <button type="button" onclick={on_run_example}>{ "▶ Run..." }</button>
                </div>
                <div class="row-item day-collapse">
                    <h5 class={if props.example.lines().count() > 1 {"button"} else {"button disabled"}} onclick={on_collapse}>
                    {
                        if props.example.lines().count() > 1 {
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
                            <pre>{&props.example}</pre>
                        }
                    } else {
                        html! {
                            <pre class={if props.example.lines().count() > 1 {"collapsed"} else {""}}>
                                {props.example.lines().next().unwrap()}
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
