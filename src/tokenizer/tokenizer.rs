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

#[cfg(test)]
mod tests {
    use crate::tokenizer::token::Token::Number;
    use crate::tokenizer::token::{ErrorType, TokenizerError};
    use crate::tokenizer::tokenizer::tokenize_number;

    #[test]
    fn tokenize_number_test_integer_number() {
        let number = String::from("12");
        let result = tokenize_number(&number);
        match result {
            Ok(token) => assert_eq!(token, Number(number)),
            Err(err) => panic!("unexpected error: {}", err),
        }
    }

    #[test]
    fn tokenize_number_test_negative_integer_number() {
        let number = String::from("-12");
        let result = tokenize_number(&number);
        match result {
            Ok(token) => assert_eq!(token, Number(number)),
            Err(err) => panic!("unexpected error: {}", err),
        }
    }

    #[test]
    fn tokenize_number_test_float_number() {
        let number = String::from("-12.15");
        let result = tokenize_number(&number);
        match result {
            Ok(token) => assert_eq!(token, Number(number)),
            Err(err) => panic!("unexpected error: {}", err),
        }
    }

    #[test]
    fn tokenize_number_test_decimal_place_number() {
        let number = String::from(".15");
        let result = tokenize_number(&number);
        match result {
            Ok(token) => assert_eq!(token, Number(number)),
            Err(err) => panic!("unexpected error: {}", err),
        }
    }

    #[test]
    fn tokenize_number_test_error() {
        let number = String::from("&12");
        let result = tokenize_number(&number);
        match result {
            Ok(token) => panic!("unexpected token: {}", token),
            Err(err) => assert_eq!(err, TokenizerError {
                error_type: ErrorType::NumberError,
            }),
        }
    }
}