use chatgpt::prelude::*;
use chatgpt::types::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Getting the API key here
    let key = std::env::args().nth(1).unwrap_or("".to_string());
    println!("Key: {}", key);
    let client = ChatGPT::new(key)?;
    let response: CompletionResponse = client
        .send_message("Name 5 blue things")
        .await?;
    println!("Response: ");
    println!("{}", response.message().content);
    Ok(())
}


