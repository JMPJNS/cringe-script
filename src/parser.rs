use crate::lexer::{Lexer}; 
use crate::ast;
use crate::token::{TokenType, Token};

struct Parser<'a> {
    lexer: &'a mut Lexer,
    curr_token: Option<Token>,
    peek_token: Option<Token>
}

impl<'a> Parser<'a> {
    fn new(&mut lexer: &'a mut Lexer) -> Self {
        let mut p = Parser{
            lexer,
            curr_token: None,
            peek_token: None
        };

        p.next_token();
        p.next_token();
        return p;
    }

    fn next_token(&mut self) {
        self.curr_token = self.peek_token;
        self.peek_token = Some(self.lexer.next_token());
    }
}