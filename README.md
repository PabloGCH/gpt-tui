DESCRIPTION
==============================

A TUI for CHATGPT writen in rust i am making to learn and practice the use of the rust language.
Feel free to use it and open issues if you think something doesn't work, should be written in a
better way, or if you have a feature request.
Since this is a learning project i will not be accepting pull requests, because i want to do add
features and refactor the code myself.

USAGE
==============================

- Clone the repo.
- Make a environment variable called OPENAI_KEY and set it to your openai key.
- Use cargo run to run the program or cargo build to build it.
if you build it you can drop the binary in your bin folder and run it from anywhere


KNOWN ISSUES
==============================

- The syntax highlighting and background for the code will sometimes not work.
This is because currently the application needs the code in the response to be in a markdown code block,
to know that it is code and not just text to set the background. And the language needs to be specified
to know what syntax to use.

TODO
==============================

- [x] Parse and color code blocks in responses.
- [ ] Add support for conversations (At least one for now).
- [ ] Add support for vertical scrolling.
- [ ] Add insert and visual mode (Controled with vim keys).
- [ ] Allow to move the cursor in the input box.
- [ ] Refactor the code.
- [ ] Add a config file to set syntect theme and openai key;
- [ ] Add support for multiple conversations.
    - [ ] Add command to remove conversations.
    - [ ] Add commands to switch between conversations.
    - [ ] Add a command to create a new conversation.







