use crossterm::event::{read, Event};

pub fn catch_events() -> crossterm::Result<()> {
    println!("LISTENING TO EVENTS");
    loop {
        // `read()` blocks until an `Event` is available
        match read()? {
            Event::Key(event) => {
                println!("{:?}", event);

            },
            Event::FocusGained => {},
            Event::FocusLost => {},
            Event::Mouse(event) => {},
            Event::Paste(data) => {},
            Event::Resize(width, height) => {},
        }
    }
    Ok(())
}
