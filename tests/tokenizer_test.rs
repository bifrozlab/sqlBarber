use sqlbarser::tokenizer::token::Token::Number;
use sqlbarser::tokenizer::token::{ErrorType, TokenizerError};
use sqlbarser::tokenizer::tokenizer::tokenize_number;

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
    let number = String::from("0015");
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