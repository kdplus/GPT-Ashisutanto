extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use reqwest;
use serde_json::json;

#[wasm_bindgen]
pub fn print_upper_string(input: &str) -> String {
    input.to_uppercase()
}

#[wasm_bindgen]
pub async fn event_create(input_text: &str) -> Result<String, JsValue> {
    let url = "https://api.openai.com/v1/chat/completions";
    let api_key = "<YOUR_API_KEY>";

    let client = reqwest::Client::new();
    let prompt_prefix = "Extract time slot from the given email and your reply must follow this four rules: \n\
        1. Reply the time duration in format 'MM/DD/YYYY HH:MM ~ HH:MM' \n\
        2. Please use 24 hour format not am and pm \n\
        3. Wrap the answer with prefix '!#?' and suffix '?#!' \n\
        4. Do not contain any extra words than the time duration \n\
        Example output: '!#?03/28/2023 10:00 ~ 17:00?#!'
        Here is the email text: ";
    let prompt_text = [prompt_prefix, input_text].join("\n");
    let response = client.post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": "gpt-3.5-turbo",
            "messages": [
                {"role": "system", "content": "Pretend to be a function that return a time duration in format 'MM/DD/YYYY HH:MM ~ HH:MM', you are not allow to contain any extra world in your reply"},
                {"role": "user", "content": prompt_text}
            ]
        }))
        .send()
        .await?;

    let text = response.text().await?;
    let json_data: serde_json::Value = serde_json::from_str(&text)
        .map_err(|err| format!("failed to parse response JSON: {:?}", err))?;



    let choices = json_data["choices"].as_array().ok_or("Missing `choices` field in response")?;

    if choices.is_empty() {
        return Err(JsValue::from_str("No response received"));
    }

    let message = &choices[0]["message"];
    let content = &message["content"].as_str().ok_or("Missing `content` field in response")?;

    Ok(content.to_string())
}

