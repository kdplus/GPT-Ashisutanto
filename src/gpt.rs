use reqwest;
use serde_json::json;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;


pub async fn ask_gpt(messages: Vec<HashMap<&str, &str>>, api_key: String) -> Result<String, JsValue> {
    let url = "https://api.openai.com/v1/chat/completions";

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
