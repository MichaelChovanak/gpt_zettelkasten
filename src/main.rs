mod gpt4 {
    pub mod api;
    pub mod models;
}

use gpt4::api::call_gpt4_api;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let prompt = "Translate the following English text to French: 'Hello, how are you?'";
    let max_tokens = 50;

    let result = call_gpt4_api(prompt, max_tokens).await?;

    println!("Generated text: {}", result);

    Ok(())
}
