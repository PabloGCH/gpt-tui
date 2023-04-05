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
fn main() {
    use syntect::easy::HighlightLines;
    use syntect::parsing::SyntaxSet;
    use syntect::highlighting::{ThemeSet, Style};
    use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};
    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ps.find_syntax_by_extension("js").unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    let s = "
        function helloWorld() {
            console.log('Hello World')
        }
    ";
    for line in LinesWithEndings::from(s) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        print!("{}", escaped);
    }
    println!("\n ")
}

