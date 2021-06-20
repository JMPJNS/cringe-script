use crate::lexer::{Lexer}; 
use crate::ast;
use crate::token::{TokenType, Token};

pub struct Parser {
    lexer: Lexer,
    curr_token: Option<Token>,
    peek_token: Option<Token>
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut p = Parser{
            lexer,
            curr_token: None,
            peek_token: None
        };

        p.next_token();
        p.next_token();
        return p;
    }

    pub fn parse_program(&mut self) -> ast::Program {
        let mut prg = ast::Program::new();

        loop {
            if let Some(i) = &self.curr_token {
                if i.token_type == TokenType::Eof {break;}
            }

            let stm = self.parse_statement();
            if let Some(i) = stm {
                prg.statements.push(i)
            }

            self.next_token();
        }

        return prg;
    }

    fn parse_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        if let Some(i) = self.curr_token.clone() {
            match i.token_type {
                TokenType::Let => return self.parse_let_statement(),
                _ => return None
            }
        } else {
            None
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn ast::Statement>> {
        if let Some(i) = self.curr_token.clone() {
            let mut stm = ast::LetStatement {
                token: i.clone(),
                name: None,
                value: None
            };

            if !self.compare_next(TokenType::Ident) {return None}
            if !self.next_token() {return None}

            let ident = ast::Identifier {
                token: self.curr_token.clone().unwrap(),
                value: self.curr_token.clone().unwrap().literal
            };
            stm.name = Some(ident);

            if !self.compare_next(TokenType::Assign) {return None}
            if !self.next_token() {return None}

            Some(Box::new(stm))
        } else {
            None
        }
    }

    fn compare_next(&self, tt: TokenType) -> bool {
        if let Some(i) = self.peek_token.clone() {
            i.token_type == tt
        } else {
            false
        }
    }

    // returns true if curr_token has a value
    fn next_token(&mut self) -> bool {
        self.curr_token = self.peek_token.clone();
        self.peek_token = Some(self.lexer.next_token());

        match self.curr_token.clone() {
            Some(_) => true,
            None => false
        }
    }
}