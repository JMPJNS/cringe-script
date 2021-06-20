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

trait Statement: Node {
    fn statement_node(&self);
}
impl Debug for dyn Statement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Statement{{{}}}", self.token_literal())
    }
}

trait Expression: Node {
    fn expression_node(&self);
}
impl Debug for dyn Expression {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Expression{{{}}}", self.token_literal())
    }
}

#[derive(Debug)]
struct Program {
    statements: Vec<Box<dyn Statement>>
}

#[derive(Debug)]
struct Identifier {
    token: Token,
    value: String
}

/// Let Statement

#[derive(Debug)]
struct LetStatement<'a> {
    token: Token,
    name: &'a Identifier,
    value: dyn Expression
}

impl<'a> Node for LetStatement<'a> {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl<'a> Statement for LetStatement<'a> {
    fn statement_node(&self) {}
}