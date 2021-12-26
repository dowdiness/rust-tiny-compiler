use compiler::tokenizer::*;
use compiler::parser::parser;

// todo "(add 2 (subtract 4 2))" のように()がネストすると動かない。
fn main() {
    let tokens = tokenizer("(add 2 4)");
    let ast = parser(tokens);
    println!("{:?}", ast);
}
