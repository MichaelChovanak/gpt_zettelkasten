use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GPT4Request {
    pub model: String,
    pub messages: Vec<Message>,
    pub max_tokens: u32,
}

#[derive(Debug, Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}
