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

    println!("starting\n\n\n\n");

    let mut lexer = lexer::Lexer::new(input.to_string());

    let mut x = lexer.next_token();

    let mut counter = 0;

    loop {
        match x.token_type {
            TokenType::Eof => {
                println!("{:?}", x);
                break
            },
            _ => {
                println!("{:?}", x);
                x = lexer.next_token();
                counter += 1;
                if counter >= 1000 {
                    lexer.next_token();
                    break
                };
            }
        }
    }

    println!("{} Tokens", counter)
}
