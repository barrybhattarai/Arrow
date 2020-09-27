#![allow(dead_code)]
//Token is a struct representing a token from the source code
#[derive(Debug)]
pub struct Token {
    content: char,
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
}

impl Token {
    pub fn tokenize(s: String) -> Result<Vec<Token>, ()> {
        let mut tokens: Vec<Token> = vec![];
        for item in s.chars() {
            tokens.push(Token {
                content: item,
                token_type: TokenType::Literal,
            })
        }
        return Ok(tokens);
    }
}
