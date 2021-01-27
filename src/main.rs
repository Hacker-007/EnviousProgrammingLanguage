use std::fs;

use lexer::{token::TokenKind, Lexer};
use parser::Parser;

mod error;
mod lexer;
mod parser;
mod span;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("src/test.envy")?;
    let (tokens, errors) = Lexer::new("src/test.envy".to_string(), input.as_bytes()).get_tokens();
    errors.iter().for_each(|error| error.report_error());
    let filtered_tokens = tokens
        .into_iter()
        .filter(|token| token.1 != TokenKind::Whitespace)
        .collect::<Vec<_>>();
    let (expressions, errors) = Parser::new(filtered_tokens).parse_program();
    errors.iter().for_each(|error| error.report_error());
    for expression in expressions {
        println!("{:#?}", expression.1);
    }

    Ok(())
}
