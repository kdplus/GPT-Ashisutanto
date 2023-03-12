extern crate wasm_bindgen;
mod gpt;

use gpt::ask_gpt;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
struct EventInfo {
    time: String,
    text: String,
    detail: String,
}

#[wasm_bindgen]
pub async fn event_create(input_text: String, api_key: &str) -> Result<JsValue, JsValue> {
    let time_prefix =
        "Extract time slot from the given email and your reply must follow this four rules: \n\
        1. Reply the time duration in format 'MM/DD/YYYY HH:MM ~ HH:MM' \n\
        2. Please use 24 hour format not am and pm \n\
        3. Wrap the answer with prefix '!#?' and suffix '?#!' \n\
        4. Do not contain any extra words than the time duration \n\
        Example output: '!#?03/28/2023 10:00 ~ 17:00?#!'
        Here is the email text: ";
    let detail_prefix = "I want to extract the infomation to create a event on google calendar, \n\
        could you help me extract event title and event detail from following email and your reply must follow these tree rules: \n\
        1. Fill the information into <Title PLACEHOLDER> <Detail PLACEHOLDER>, and reply in this format: \n\
            <Title PLACEHOLDER>/!#?/<Detail PLACEHOLDER> \n\
        2. Please extract important information like meeting link into detail placeholder \n\
        3. Don't reply extra word rather than these two information \n\
        Here is the email:";
    let time_prompt_text = format!("{}\n{}", time_prefix, input_text);
    let detail_prompt_text = format!("{}\n{}", detail_prefix, input_text);

    let time_messages = vec![
        HashMap::from([
            ("role", "system"),
            ("content", "Pretend to be a function that return a time duration in format 'MM/DD/YYYY HH:MM ~ HH:MM',\
                you are not allow to contain any extra world in your reply"),
        ]),
        HashMap::from([
            ("role", "user"),
            ("content", &time_prompt_text),
        ])
    ];
    let detail_messages = vec![HashMap::from([
        ("role", "user"),
        ("content", &detail_prompt_text),
    ])];

    let time_slot = ask_gpt(time_messages, api_key.to_owned()).await?;
    let title_detail = ask_gpt(detail_messages, api_key.to_owned()).await?;

    let (title, detail) = match title_detail
        .split("/!#?/")
        .map(|s| s.trim())
        .collect::<Vec<&str>>()[..]
    {
        [t, d] => (t.to_owned(), d.to_owned()),
        _ => ("".to_owned(), "".to_owned()),
    };

    let event_info = EventInfo {
        time: time_slot,
        text: title,
        detail: detail,
    };
    let event_info_js = to_value(&event_info).unwrap();
    Ok(event_info_js)
}
