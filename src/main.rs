use spinners::{Spinner, Spinners};
use std::io::{stdin, stdout, Write, Result};
use crate::gpt4::models::Message;
mod gpt4;

async fn interactive_loop() {
    clear_terminal();
    let mut message_history: Vec<Message> = Vec::new();
    let starter_prompt = Message {
        role: "system".to_string(),
        content: "You are a helpful assistant who helps keep track of notes".to_string(),
    };
    message_history.push(starter_prompt);
    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut prompt = String::new();
        stdin()
            .read_line(&mut prompt)
            .expect("Failed to read line");

        println!("");

        let mut sp = Spinner::new(Spinners::Dots10, "\t\tOpenAI is Thinking".into());

        let mut response = String::new();        
        match gpt4::api::call_gpt4_api(&prompt, &mut message_history, 500).await {
            Ok(res) => response=res,
            Err(e) => eprintln!("Error: {:?}", e),
        };
        sp.stop();
        clear_line().unwrap();

        println!("{}\n", response.to_string());
    }
}

fn clear_terminal() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    stdout().flush().unwrap();
}

fn clear_line() -> Result<()> {
    let stdout = stdout();
    let mut handle = stdout.lock();

    // move the cursor to the beginning of the line
    write!(handle, "\r")?;

    // clear the line
    write!(handle, "\x1b[2K")?;

    // move the cursor back to the beginning of the line
    write!(handle, "\r")?;

    handle.flush()?;

    Ok(())
}

#[tokio::main]
async fn main() {
    interactive_loop().await;
}

