use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};


pub fn parse_markdown(text: &str) -> String {
    //PARSED TEXT
    let mut parsed_text = String::new();
    parsed_text.push_str(text);
    //BETWEEN FIRST AND SECOND INDEX IS THE CODE
    //GETS THE FIRST INDEX OF THE CODE
    let mut first_index :Option<usize> = text.find("```");
    let mut second_index :Option<usize> = text[first_index.unwrap_or(0) + 3..].find("```");

    while first_index != None && second_index != None {
        //GETS THE SECOND INDEX OF THE CODE
        //SPLITS THE TEXT INTO 3 PARTS (BEFORE, CODE, AFTER)
        let before = parsed_text[..first_index.unwrap()].to_string();
        let code = parsed_text[first_index.unwrap() + 3..first_index.unwrap() + second_index.unwrap() + 3].to_string();
        let after = parsed_text[first_index.unwrap() + second_index.unwrap() + 6..].to_string();
        //SETS THE SYNTAX
        let syntax_set = SyntaxSet::load_defaults_newlines();
        //SETS THE THEME
        let theme_set = ThemeSet::load_defaults();
        //TRIES TO GET THE SYNTAX
        let lang = code.split("\n").next().unwrap_or("none");
        //SETS THE SYNTAX
        let syntax = syntax_set.find_syntax_by_name(lang).unwrap_or_else(|| syntax_set.find_syntax_plain_text());
        //SETS THE HIGHLIGHTER
        let mut highlight = HighlightLines::new(syntax, &theme_set.themes["base16-eighties.dark"]);
        //HIGHLIGHTS THE CODE
        let mut highlighted_code = String::new();
        //SETS THE BACKGROUND COLOR
        highlighted_code.push_str("\x1b[48;5;0m\n");
        for line in LinesWithEndings::from(&code) {
            let ranges: Vec<(Style, &str)> = highlight.highlight_line(line, &syntax_set).unwrap();
            let color_line_code = "\x1b[K\t".to_string();
            let mut escaped = as_24_bit_terminal_escaped(&ranges[..], false);
            escaped.insert_str(0, &color_line_code);
            highlighted_code.push_str(&escaped);
        }
        //STOPS BACKGROUND COLORING
        highlighted_code.push_str("\x1b[0m");
        highlighted_code.push_str("\n");
        //SETS THE NEW TEXT
        parsed_text = before + &highlighted_code + &after;
        //GETS THE FIRST AND SECOND INDEX OF THE CODE
        //IF ONE OF THEM IS NONE, THE CODE IS OVER
        first_index = parsed_text.find("```");
        second_index = parsed_text[first_index.unwrap_or(0) + 3..].find("```");
    }
    //new_text.push_str(&text);
    return parsed_text;
}


/*
   let _v = text.split("```");
   let mut _i = 0;
   for st in _v {
   new_text.push_str(&st);
   let debug = "\nSTRING INDEX: ".to_string() + &_i.to_string() + "\n";
   new_text.push_str(&debug);
   _i += 1;
   }
   */
