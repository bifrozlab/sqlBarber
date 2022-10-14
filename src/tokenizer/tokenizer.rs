use nom::character::is_digit;
use crate::tokenizer::token::{ErrorType, Token, TokenizerError};
use crate::tokenizer::token::Token::Number;

pub fn tokenize_number(text: &String) -> Result<Token, TokenizerError> {
    let mut number = String::from("");
    let mut text_iter = text.as_bytes().iter().peekable();

    while let Some(&c) = text_iter.next() {
        if is_digit(c) {
            number.push(char::from(c));
            continue;
        }

        match text_iter.peek() {
            Some(&&next_c) if is_digit(next_c) && is_number_prefix(c) => number.push(char::from(c)),
            _ => return Err(TokenizerError {
                error_type: ErrorType::NumberError
            }),
        }
    }

    Ok(Number(number))
}

fn is_number_prefix(c: u8) -> bool {
    match c {
        b'-' => true,
        b'.' => true,
        _ => false
    }
}
