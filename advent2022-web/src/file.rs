use std::str::from_utf8;

use gloo_file::callbacks::read_as_bytes;
use gloo_file::File;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FileProps {
    pub day_num: usize,
    pub file_load_callback: Callback<String>,
}

#[function_component]
pub fn FileUpload(props: &FileProps) -> Html {
    let file_reader = use_state(|| None);
    let on_file_upload = {
        let file_reader = file_reader.clone();
        let file_load_callback = props.file_load_callback.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut gloo_files: Vec<File> = Vec::new();
            if let Some(files) = input.files() {
                let files = js_sys::try_iter(&files)
                    .unwrap()
                    .unwrap()
                    .map(|v| web_sys::File::from(v.unwrap()))
                    .map(File::from);
                gloo_files.extend(files);
            }
            if gloo_files.len() > 1 {
                panic!("more than  one file received")
            }
            let gloo_file = gloo_files.pop().unwrap();
            let file_name = gloo_file.name();
            log::info!("loading file '{}'...", file_name);
            let file_load_callback = file_load_callback.clone();
            let reader = read_as_bytes(&gloo_file, move |blob_bytes| {
                let read_string = from_utf8(&blob_bytes.unwrap()).unwrap().to_owned();
                file_load_callback.emit(read_string)
            });
            file_reader.set(Some((file_name, reader)))
        })
    };
    let file_upload_id = format!("file-upload-day-{}", props.day_num);
    html! {
        <div class="row-item day-file">
            <label for={file_upload_id.clone()} class="custom-file-upload">{
                if file_reader.is_none() {
                    {"📄 Upload..."}
                } else {
                    {"Loading..."}
                }
            }</label>
            <input id={file_upload_id.clone()} type="file" onchange={on_file_upload} />
        </div>
    }
}
