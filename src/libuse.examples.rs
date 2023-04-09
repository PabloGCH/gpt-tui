

/*
//GETTING KEY FROM ARGUMENTS AND ASKING FOR PROMPTS
fn main() {
    //GETS THE OPEN AI KEY FROM THE COMMAND LINE
    //let open_ai_key :String = std::env::args().nth(1).unwrap();
    //CREATES A NEW OPEN AI CLIENT
    //let client = openai::Client::new(open_ai_key);
    println!("ENTER A PROMPT: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    println!("RESPONSE: {}", input);
}
*/


//SYNTAX HIGHLIGHT TEST
/*
use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

fn main() {
    // Load these once at the start of your program
    let syntax_set = SyntaxSet::load_defaults_newlines();
    //Load the theme
    let theme_set = ThemeSet::load_defaults();
    // Find a syntax definition based on the path
    let syntax = syntax_set.find_syntax_by_extension("js").unwrap();
    // Create a HighlightLines struct that can be used for actually highlighting source code
    let mut highlight = HighlightLines::new(syntax, &theme_set.themes["base16-eighties.dark"]);
    //Create a snippet
    let s = "
        function helloWorld() {
            console.log('Hello World')
        }
    ";
    let snippet = LinesWithEndings::from(s);
    // Highlight the snippet
    println!("\x1b[48;5;0m");
    for line in snippet {
        let ranges: Vec<(Style, &str)> = highlight.highlight_line(line, &syntax_set).unwrap();
        //let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
        
        let color_line_code = "\x1b[K".to_string();
        let mut escaped = as_24_bit_terminal_escaped(&ranges[..], false);
        escaped.insert_str(0, &color_line_code);

        print!("{}", escaped);
    }
    println!("\x1b[0m");
    println!("");
}
*/

