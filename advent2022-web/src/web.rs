use std::collections::HashMap;
use std::rc::Rc;

use yew::prelude::*;
use yew::services::reader::{File, FileData, ReaderService, ReaderTask};
use yew::Properties;

use advent2022_lib::ParseResult;

#[derive(Properties, Clone)]
pub struct DayProps {
    pub day_num: usize,
    pub title: &'static str,
    pub example: String,
    #[prop_or_default]
    pub show_input: bool,
    pub both_func: Rc<dyn Fn(&str) -> ParseResult<(String, String)>>,
    pub text_format: (&'static str, &'static str),
    #[prop_or_default]
    pub messages: Vec<String>,
}

type FileName = String;

pub enum Msg {
    RunExample,
    File(Option<File>),
    Loaded(FileData),
    Collapse,
}

pub struct Day {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    props: DayProps,
    tasks: HashMap<FileName, ReaderTask>,
}

impl Component for Day {
    type Message = Msg;
    type Properties = DayProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            tasks: HashMap::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::RunExample => {
                log::info!("Running Example");
                let day_func = self.props.both_func.clone();
                let result = day_func(&self.props.example);
                match result {
                    Err(_e) => {
                        log::error!("parsing error...");
                        self.props.messages = vec!["Parsing error, please try again...".to_owned()]
                    }
                    Ok(answer) => {
                        let part1 = format!(
                            "Part 1: {}",
                            self.props.text_format.0.replace("{answer}", &answer.0)
                        );
                        let part2 = format!(
                            "Part 2: {}",
                            self.props.text_format.1.replace("{answer}", &answer.1)
                        );
                        log::info!("{}", part1);
                        log::info!("{}", part2);
                        self.props.messages = vec![part1, part2];
                    }
                }
                true
            }
            Msg::File(Some(file)) => {
                let file_name = file.name();
                log::info!("loading file '{}'...", file_name);
                let task = {
                    let callback = self.link.callback(Msg::Loaded);
                    ReaderService::read_file(file, callback).unwrap()
                };
                self.tasks.insert(file_name, task);
                false
            }
            Msg::File(None) => {
                log::warn!("file upload failed");
                false
            }
            Msg::Loaded(file) => {
                let s = match std::str::from_utf8(&file.content) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                }
                .to_owned();
                self.props.example = s;
                let _ = self.tasks.remove(&file.name).expect("no file removed");
                log::info!("loaded file '{}'...", file.name);
                true
            }
            Msg::Collapse => {
                self.props.show_input = !self.props.show_input;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        if self.props.messages != props.messages {
            self.props.messages = props.messages;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let file_upload_id = format!("file-upload-day-{}", self.props.day_num);
        html! {
            <section class={if self.props.day_num&1 != 0 { "day-odd" } else { "day-even" }}>
                <div class="row">
                    <div class="row-item day-key"><h4>{"Day "}{self.props.day_num}{":"}</h4></div>
                    <div class="row-item day-title"><h2><em>{&self.props.title}</em></h2></div>
                    <div class="row-item day-url"><a href={format!("https://github.com/droogmic/advent2022/blob/master/advent2022-lib/src/day{:02}.rs", self.props.day_num)}>{"Source Code"}</a></div>
                </div>
                <div class="row row-reverse">
                    <div class="row-item day-file">
                        <label for={file_upload_id.clone()} class="custom-file-upload">{"📄 Upload..."}</label>
                        <input id={file_upload_id.clone()} type="file" onchange=self.link.callback(move |value| {
                            if let ChangeData::Files(files) = value {
                                assert_eq!(files.length(), 1);
                                let file = files
                                    .get(0)
                                    .unwrap();
                                return Msg::File(Some(file))
                            }
                            Msg::File(None)
                        }) />
                    </div>
                    <div class="row-item day-run">
                        <button type="button" onclick=self.link.callback(|_| Msg::RunExample)>{ "▶ Run..." }</button>
                    </div>
                    <div class="row-item day-collapse">
                        <h5 class={if self.props.example.lines().count() > 1 {"button"} else {"button disabled"}} onclick=self.link.callback(|_| Msg::Collapse)>
                        {
                            if self.props.example.lines().count() > 1 {
                                if self.props.show_input {
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
                        if self.props.show_input {
                            html! {
                                <pre>{&self.props.example}</pre>
                            }
                        } else {
                            html! {
                                <pre class={if self.props.example.lines().count() > 1 {"collapsed"} else {""}}>
                                    {self.props.example.lines().next().unwrap()}
                                </pre>
                            }
                        }
                    }
                    {
                        for self.props.messages.iter().map(|message| {
                            html! {
                                <p>{message}</p>
                            }
                        })
                    }
            </section>
        }
    }
}
