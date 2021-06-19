#[path = "../token/token.rs"] pub mod token;

pub struct Lexer {
    pub input: String,
    pub position: usize, // current position
    pub read_position: usize, // next position,
    pub current_char: char
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            current_char: 0.into()
        };

        l.read_char();

        return l;
    }

    pub fn next_token(&mut self) -> token::Token {
        self.skip_whitespace();
        let mut literal: String = self.current_char.into();
        let mut token_type: token::TokenType;

        match self.current_char {
            '=' => {
                if self.peek_char() == '=' {
                    token_type = token::TokenType::Eq;
                    self.read_char();
                    literal = "==".into();
                } else {
                    token_type = token::TokenType::Assign;
                }
            },
            ';' => {token_type = token::TokenType::Semicolon},
            '(' => {token_type = token::TokenType::Lparen},
            ')' => {token_type = token::TokenType::Rparen},
            ',' => {token_type = token::TokenType::Comma},
            '+' => {token_type = token::TokenType::Plus},
            '-' => {token_type = token::TokenType::Minus},
            '!' => {
                if self.peek_char() == '=' {
                    token_type = token::TokenType::NotEq;
                    self.read_char();
                    literal = "!=".into();
                } else {
                    token_type = token::TokenType::Bang;
                }
            },
            '*' => {token_type = token::TokenType::Asterisk},
            '/' => {token_type = token::TokenType::Slash},
            '{' => {token_type = token::TokenType::Lbrace},
            '}' => {token_type = token::TokenType::Rbrace},
            '<' => {token_type = token::TokenType::Lt},
            '>' => {token_type = token::TokenType::Gt},
            _ => {
                if is_letter(self.current_char) {
                    return token::Token{
                        literal: self.read_identifier(),
                        token_type: token::lookup_ident(&literal)
                    }
                } else if is_digit(self.current_char) {
                    return token::Token{
                        literal: self.read_number(),
                        token_type: token::TokenType::Int
                    }
                } else {
                    token_type = token::TokenType::Illegal;
                }
            }
        }

        if self.current_char == 0.into() {
            token_type = token::TokenType::Eof;
        }

        self.read_char();

        token::Token {
            token_type,
            literal
        }
    }

    fn read_char(&mut self) {
        self.current_char = self.peek_char();

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            return 0.into()
        } else {
            return self.input.chars()
                                .nth(self.read_position).unwrap_or(0.into())
        }
    }

    fn read_identifier(&mut self) -> String {
        let init_pos = self.position;

        while is_letter(self.current_char) {
            self.read_char();
        };

        let ret = (&self.input[init_pos..self.position]).to_string();
        return ret;
    }

    fn read_number(&mut self) -> String {
        let init_pos = self.position;

        while is_digit(self.current_char) {
            self.read_char();
        };

        let ret = (&self.input[init_pos..self.position]).to_string();
        return ret;
    }

    fn skip_whitespace(&mut self) {
        while is_whitespace(self.current_char) {
            self.read_char();
        };
    }
}

fn is_letter(ch: char) -> bool {
    match ch {
        'A'..='Z' | 'a'..='z' | '_' => true,
        _ => false
    }
}

fn is_digit(ch: char) -> bool {
    match ch {
        '0'..='9' => true,
        _ => false
    }
}

fn is_whitespace(ch: char) -> bool {
    match ch {
        ' ' | '\n' | '\t' | '\r' => true,
        _ => false
    }
}

#[test]
fn test_next_token() {
}