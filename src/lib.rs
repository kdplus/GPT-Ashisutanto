extern crate wasm_bindgen;
mod gpt;

use gpt::ask_gpt;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn event_create(input_text: &str) -> Result<String, JsValue> {
    let prompt_prefix =
        "Extract time slot from the given email and your reply must follow this four rules: \n\
        1. Reply the time duration in format 'MM/DD/YYYY HH:MM ~ HH:MM' \n\
        2. Please use 24 hour format not am and pm \n\
        3. Wrap the answer with prefix '!#?' and suffix '?#!' \n\
        4. Do not contain any extra words than the time duration \n\
        Example output: '!#?03/28/2023 10:00 ~ 17:00?#!'
        Here is the email text: ";
    let prompt_text = [prompt_prefix, input_text].join("\n");

    let messages = vec![
        HashMap::from([
            ("role", "system"),
            ("content", "Pretend to be a function that return a time duration in format 'MM/DD/YYYY HH:MM ~ HH:MM',\
                you are not allow to contain any extra world in your reply"),
        ]),
        HashMap::from([
            ("role", "user"),
            ("content", &prompt_text),
        ])
    ];

    ask_gpt(messages).await
}
