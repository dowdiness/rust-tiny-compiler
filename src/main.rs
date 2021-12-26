use compiler::tokenizer::*;
// use compiler::parser::parser;

fn main() {
    let tokens = tokenizer("(add 2 (subtract 4 2))");
    // let ast = parser(&mut vec!(Token::new(TokenType::Number, "22")));
    println!("{:?}", tokens);
}
