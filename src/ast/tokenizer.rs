use nom::character::is_digit;
use crate::ast::token::{ErrorType, Token, TokenizerError};
use crate::ast::token::Token::Number;

pub fn tokenize_number(text: &String) -> Result<Token, TokenizerError> {
    let mut number = String::from("");
    for &item in text.as_bytes().iter() {
        if is_digit(item) {
            number.push(char::from(item))
        } else {
            return Err(TokenizerError {
                error_type: ErrorType::NumberError,
            });
        }
    }

    Ok(Number(number))
}

mod tests {
    use crate::ast::token::Token::Number;
    use crate::ast::token::{ErrorType, TokenizerError};
    use crate::ast::tokenizer::tokenize_number;

    #[test]
    fn tokenize_number_test_simple_number() {
        let number = String::from("12");
        let result = tokenize_number(&number);
        match result {
            Ok(token) => assert_eq!(token, Number(number)),
            Err(err) => panic!("unexpected error: {}", err),
        }
    }

    #[test]
    fn tokenize_number_test_error() {
        let number = String::from("aaa");
        let result = tokenize_number(&number);
        match result {
            Ok(token) => panic!("unexpected token: {}", token),
            Err(err) => assert_eq!(err, TokenizerError {
                error_type: ErrorType::NumberError,
            }),
        }
    }
}