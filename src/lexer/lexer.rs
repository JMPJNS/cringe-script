#[path = "../token/token.rs"] pub mod token;

pub struct Lexer {
    pub input: String,
    pub position: usize, // current position
    pub read_position: usize, // next position,
    pub current_char: char
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input,
            position: 0,
            read_position: 0,
            current_char: 0.into()
        }
    }

    pub fn next_token(&mut self) -> token::Token {
        self.read_char();

        let literal: String;
        let mut token_type: token::TokenType;

        match self.current_char {
            'A'..='Z' | 'a'..='z' => literal = self.read_identifier(),
            _ => literal = self.current_char.to_string()
        };

        match &literal[..] {
            "=" => {token_type = token::TokenType::Eq},
            ";" => {token_type = token::TokenType::Semicolon},
            "(" => {token_type = token::TokenType::Lparen},
            ")" => {token_type = token::TokenType::Rparen},
            "," => {token_type = token::TokenType::Comma},
            "+" => {token_type = token::TokenType::Plus},
            "-" => {token_type = token::TokenType::Minus},
            "!" => {token_type = token::TokenType::Bang},
            "*" => {token_type = token::TokenType::Asterisk},
            "/" => {token_type = token::TokenType::Slash},
            "{" => {token_type = token::TokenType::Lbrace},
            "}" => {token_type = token::TokenType::Rbrace},
            "let" => {token_type = token::TokenType::Let},
            _ => {token_type = token::TokenType::Illegal}
        }

        if self.current_char == 0.into() {
            token_type = token::TokenType::Eof;
        }

        token::Token {
            token_type,
            literal
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.current_char = 0.into()
        } else {
            self.position = self.read_position;
            self.read_position += 1;

            self.current_char = self.input.chars()
                                .nth(self.read_position).unwrap_or(0.into())
        }
    }

    fn read_identifier(&mut self) -> String {
        let init_pos = self.read_position;

        fn is_letter(ch: char) -> bool {
            match ch {
                'A'..='Z' | 'a'..='z' => true,
                _ => false
            }
        }

        while is_letter(self.current_char) {
            self.read_char();
        };

        return (&self.input[init_pos..self.read_position]).to_string();
    }
}

#[test]
fn test_next_token() {
}