#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Illegal,
    Eof,
    // Identifiers+Literals
    Ident,
    Int,
    //Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Eq,
    NotEq,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
    Whitespace,
    Newline,
    Tab
}

pub fn lookup_ident(i: &String) -> TokenType {
    match &(i.to_lowercase())[..] {
        "fn" => TokenType::Function,
        "let" => TokenType::Let,
        "true" => TokenType::True,
        "false" => TokenType::False,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "return" => TokenType::Return,
        _ => TokenType::Ident
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}