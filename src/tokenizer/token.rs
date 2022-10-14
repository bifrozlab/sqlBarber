use std::fmt::Display;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct TokenizerError {
    pub error_type: ErrorType,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorType {
    NumberError
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorType::NumberError => write!(
                f,
                "tokenized number error",
            )
        }
    }
}

impl Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.error_type
        )
    }
}

impl std::error::Error for TokenizerError {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    /// An end-of-file marker, not a real token
    EOF,
    /// A keyword (like SELECT) or an optionally quoted SQL identifier
    Word(Word),
    /// An numeric literal
    Number(String),
    /// Single quoted string: i.e: 'string'
    SingleQuotedString(String),
    /// Whitespace (space, tab, etc)
    Whitespace(Whitespace),
    /// Equality operator `=`
    Eq,
    /// Not Equals operator `<>` (or `!=` in some dialects)
    Neq,
    /// Less Than operator `<`
    Lt,
    /// Greater Than operator `>`
    Gt,
    /// Less Than Or Equals operator `<=`
    LtEq,
    /// Greater Than Or Equals operator `>=`
    GtEq,
    /// Period (used for compound identifiers or projections into nested types)
    Period,
    /// Comma `,`
    Comma,
    /// SemiColon `;` used as separator for COPY and payload
    SemiColon,
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::EOF =>
                write!(
                    f,
                    "EOF",
                ),
            Token::Word(word) => write!(
                f,
                "{}",
                word
            ),
            Token::Number(number) => write!(
                f,
                "{}",
                number
            ),
            Token::SingleQuotedString(str) => write!(
                f,
                "{}",
                str
            ),
            Token::Whitespace(space) => write!(
                f,
                "{}",
                space
            ),
            Token::Eq => write!(
                f,
                "=",
            ),
            Token::Neq => write!(
                f,
                "!=",
            ),
            Token::Lt => write!(
                f,
                "<",
            ),
            Token::Gt => write!(
                f,
                ">",
            ),
            Token::LtEq => write!(
                f,
                "<=",
            ),
            Token::GtEq => write!(
                f,
                ">=",
            ),
            Token::Period => write!(
                f,
                ".",
            ),
            Token::Comma => write!(
                f,
                ",",
            ),
            Token::SemiColon => write!(
                f,
                ";",
            ),
        }
    }
}


/// A keyword (like SELECT) or an optionally quoted SQL identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Word {
    /// The value of the token, without the enclosing quotes, and with the
    /// escape sequences (if any) processed (TODO: escapes are not handled)
    pub value: String,
}

impl Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Whitespace {
    Space,
    Newline,
    Tab,
    SingleLineComment { comment: String, prefix: String },
    MultiLineComment(String),
}

impl Display for Whitespace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Whitespace::Space => write!(f, " "),
            Whitespace::Newline => write!(f, "\n"),
            Whitespace::Tab => write!(f, "\t"),
            Whitespace::SingleLineComment { comment, prefix } => write!(f, "{} {}", *prefix, comment),
            Whitespace::MultiLineComment(text) => write!(f, "{}", text),
        }
    }
}