use crate::gpt4::models::{GPT4Request, Message};
use reqwest::Error;
use serde_json::Value;
use std::env;

pub async fn call_gpt4_api(prompt: &str, max_tokens: u32) -> Result<String, Error> {
    let api_key = env::var("OPENAI_API_KEY").unwrap_or_else(|_| String::from("<YOUR_API_KEY>"));
    let url = "https://api.openai.com/v1/chat/completions";

    let client = reqwest::Client::new();
    let request_payload = GPT4Request {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: "You are a helpful assistant.".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            },
        ],
        max_tokens,
    };

    let response_json: Value = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_payload)
        .send()
        .await?
        .json()
        .await?;

    println!("JSON Response: {:?}", response_json);

    let generated_text = response_json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("No result")
        .to_string();

    Ok(generated_text)
}
