use std::env::args;
use chatgpt::prelude::*;
use futures_util::StreamExt;
use std::io::{stdout, Write};

static mut BUFFER: String = String::new();

fn update_buffer(new_string: String) {
    unsafe {
        BUFFER.push_str(new_string.as_str());
        std::process::Command::new("clear").status().unwrap();
        print!("{}", BUFFER);
        // Manually flushing the standard output, as `print` macro does not do that
        stdout().lock().flush().unwrap();
    }
}

async fn get_response(query :String, client: ChatGPT) {
    let stream = client
        .send_message_streaming(query.as_str())
        .await;
    // Iterating over stream contents
    stream.unwrap().for_each(|each| async move {
        match each {
            ResponseChunk::Content { delta, response_index: _,} => {
                update_buffer(delta);
                println!("")
            }
            _ => {}
        }
    })
    .await;
}

#[tokio::main]
async fn main() -> Result<()> {
    // Creating a client
    let key = args().nth(1).unwrap();
    let client = ChatGPT::new(key)?;
    get_response("Give me a hello world program in 3 different languages".to_string(), client).await;
    Ok(())
}
