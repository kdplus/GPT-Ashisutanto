use lazy_static::lazy_static;
use reqwest;
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use wasm_bindgen::prelude::*;

lazy_static! {
    static ref API_KEY: String = {
        // let api_key = fs::read_to_string("./api_key.txt").expect("Failed to read API key file");
        let api_key = "<YOUR-API-KEY>";
        api_key.trim().to_owned()
    };
}

pub async fn ask_gpt(messages: Vec<HashMap<&str, &str>>) -> Result<String, JsValue> {
    let url = "https://api.openai.com/v1/chat/completions";
    let api_key = API_KEY.as_str();

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": "gpt-3.5-turbo",
            "messages": messages}))
        .send()
        .await?;

    let text = response.text().await?;
    let json_data: serde_json::Value = serde_json::from_str(&text)
        .map_err(|err| format!("failed to parse response JSON: {:#?}", err))?;
    let choices = json_data["choices"]
        .as_array()
        .ok_or("Missing `choices` field in response")?;

    if choices.is_empty() {
        return Err(JsValue::from_str("No response received"));
    }

    let content = &choices[0]["message"]["content"]
        .as_str()
        .ok_or("Missing `content` field in response")?;

    Ok(content.to_string())
}
