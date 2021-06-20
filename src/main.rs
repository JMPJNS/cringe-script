mod token;
mod lexer;
mod parser;
mod ast;
use token::{TokenType, Token};

fn main() {
//     let input = 
// "let five=5;
// let ten = 10;
// let x = 5 == 5;
// let y = 7!=7;
// let add = fn(x, y) {
//     if(x>y) {
//         return x+y;
//     } else {
//         return x+y;
//     }
// };

// let result = add(five, ten);";

    let input = "let five = 10;";

    println!("starting\n\n");

    let mut lexer = lexer::Lexer::new(input.to_string());
    let mut parser = parser::Parser::new(lexer);
    parser.parse_program();
}
