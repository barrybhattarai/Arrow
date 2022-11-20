#![allow(dead_code)]
//Token is a struct representing a token from the source code
#[derive(Debug)]
pub struct Token {
    content: String,
    token_type: TokenType,
}

#[derive(Debug)]
//TokenType is an enum of types of token
pub enum TokenType {
    Expression,
    Operator,
    Keyword,
    InvalidToken,
    Literal,
    OpenBrace,
    CloseBrace,
    Whitespace,
    Unknown,
}

impl Token {
    pub fn tokenize(s: String) -> Result<Vec<Token>, ()> {
        let mut tokens: Vec<Token> = vec![];
        for item in s.chars() {
            let token_type: TokenType;
            token_type = if item.is_ascii_digit() {
                TokenType::Literal
            } else {
                match item {
                    ' ' => TokenType::Whitespace,
                    '+' | '-' | '*' | '/' => TokenType::Operator,
                    '(' => TokenType::OpenBrace,
                    ')' => TokenType::CloseBrace,

                    _ => TokenType::Unknown,
                }
            };
            tokens.push(Token {
                content: item.to_string(),
                token_type: token_type,
            })
        }

        return Ok(tokens);
    }
}


