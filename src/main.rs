use spinners::{Spinner, Spinners};
use std::io::{stdin, stdout, Write};
use crate::gpt4::models::Message;
mod gpt4;

// Interact with the user and call the GPT-4 API
async fn interactive_loop(message_history: &mut Vec<Message>) {
    clear_terminal();
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
        match gpt4::api::call_gpt4_api(&prompt, message_history, 500).await {
            Ok(res) => response=res,
            Err(e) => eprintln!("Error: {:?}", e),
        };
        sp.stop();
        clear_line().unwrap();

        println!("{}\n", response.to_string());
    }
}

// Clear the terminal screen
fn clear_terminal() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    stdout().flush().unwrap();
}

// Clear the current line in the terminal
fn clear_line() -> std::io::Result<()> {
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

// Main function
#[tokio::main]
async fn main() {
    let mut message_history: Vec<Message> = Vec::new();
    let starter_prompt = Message {
        role: "system".to_string(),
        content: "You are a helpful assistant who helps keep track of notes".to_string(),
    };
    message_history.push(starter_prompt);
    interactive_loop(&mut message_history).await;
}

