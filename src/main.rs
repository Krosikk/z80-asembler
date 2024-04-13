
use std::fs;

pub mod lexer;
pub mod parser;
fn main() {
    let mut file_path =  String::new();
    std::io::stdin().read_line(&mut file_path).expect("Should  have been able to read input");
    dbg!(&file_path);
    let input   = fs::read_to_string(file_path.trim())
    .expect("Should have been able to read the file");
    
    dbg!{input.trim()};
    let tokens = lexer::lexer::lexer_main(input.as_str());
    dbg!(&tokens);
    parser::parser::parser_main(tokens);
}
