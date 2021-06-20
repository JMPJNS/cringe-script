use crate::token::{Token, TokenType};
use core::fmt::Debug;

// Types

trait Node {
    fn token_literal(&self) -> String;
}
impl Debug for dyn Node {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Node{{{}}}", self.token_literal())
    }
}

pub trait Statement: Node {
    fn statement_node(&self);
}
impl Debug for dyn Statement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Statement{{{}}}", self.token_literal())
    }
}

pub trait Expression: Node {
    fn expression_node(&self);
}
impl Debug for dyn Expression {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Expression{{{}}}", self.token_literal())
    }
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>
}
impl Program {
    pub fn new() -> Self {
        let v: Vec<Box<dyn Statement>> = Vec::new();
        Program {
            statements: v
        }
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String
}

/// Let Statement

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Option<Identifier>,
    pub value: Option<Box<dyn Expression>>
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}