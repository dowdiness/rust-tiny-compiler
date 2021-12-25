use regex::Regex;

pub type OffsetSize = u32;

pub struct Offset {
    pub begin: OffsetSize,
    pub end: OffsetSize,
}

#[derive(Debug, PartialEq)]
enum TokenType {
    Parenthesis,
    Name,
    Number,
    String,
}

#[derive(Debug)]
struct Token {
    r#type: TokenType,
    value: String,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.value == other.value
    }
}

impl PartialEq<TokenType> for Token {
    fn eq(&self, other: &TokenType) -> bool {
        self.r#type == *other
    }
}

impl PartialEq<Token> for TokenType {
    fn eq(&self, other: &Token) -> bool {
        *self == other.r#type
    }
}

fn tokenizer(input: &str) -> Vec<Token> {
    let bytes = input.as_bytes();

    let mut tokens = Vec::new();
    let mut current = 0;

    while current < bytes.len() {
        let mut char = bytes[current] as char;

        // TokenType::Parenthesis
        if char == '(' {
            tokens.push(Token {
                r#type: TokenType::Parenthesis,
                value: String::from("("),
            });
            current += 1;
            continue;
        }
        if char == ')' {
            tokens.push(Token {
                r#type: TokenType::Parenthesis,
                value: String::from(")"),
            });
            current += 1;
            continue;
        }

        // Skip whitespace
        if char.is_whitespace() {
            current += 1;
            continue;
        }

        // TokenType::Number
        let numbers = Regex::new(r"[0-9]").unwrap();
        if numbers.is_match(&char.to_string()) {
            let mut value = "".to_string();
            while numbers.is_match(&char.to_string()) {
                value.push_str(&char.to_string());
                current += 1;
                char = bytes[current] as char;
            }
            tokens.push(Token {
                r#type: TokenType::Number,
                value: String::from(value)
            });
            continue;
        }

        // TokenType::String
        if char == '"' {
            let mut value = "".to_string();
            current += 1;
            char = bytes[current] as char;
            while char != '"' {
                value.push_str(&char.to_string());
                current += 1;
                char = bytes[current] as char;
            }
            current += 1;
            tokens.push(Token {
                r#type: TokenType::String,
                value: String::from(value)
            });
            continue;
        }

        // TokenType::Name
        let letters = Regex::new(r"[a-z]").unwrap();
        if letters.is_match(&char.to_string()) {
            let mut value = "".to_string();
            while letters.is_match(&char.to_string()) {
                value.push_str(&char.to_string());
                current += 1;
                char = bytes[current] as char;
            }
            tokens.push(Token {
                r#type: TokenType::Name,
                value: String::from(value)
            });
            continue;
        }

        unreachable!();
    }
    // pattern match でのオフセットを使った実装がしたい
    //   match bytes[current] as char {
    //     '(' => {
    //       tokens.push(Token {
    //         r#type: TokenType::Parenthesis,
    //         value: String::from("("),
    //       });
    //     ')' => {
    //       tokens.push(Token {
    //         r#type: TokenType::Parenthesis,
    //         value: String::from("("),
    //       });
    //     },
    // char if char.is_whitespace() => continue,
    // ここの書き方がわからない
    // 0 ... 9 =>
    //     _ => {
    //       todo!();
    //       break;
    //     }
    //   }
    // }

    return tokens;
}

fn main() {
    let tokens = tokenizer("(add 2 (subtract 4 2))");
    println!("{:?}", tokens);
}


#[test]
fn token() {
    let input  = "(add 2 (subtract 4 2))";
    let tokens = vec!(
        Token { r#type: TokenType::Parenthesis, value: String::from("(")},
        Token { r#type: TokenType::Name, value: String::from("add")},
        Token { r#type: TokenType::Number, value: String::from("2")},
        Token { r#type: TokenType::Parenthesis, value: String::from("(")},
        Token { r#type: TokenType::Name, value: String::from("subtract")},
        Token { r#type: TokenType::Number, value: String::from("4")},
        Token { r#type: TokenType::Number, value: String::from("2")},
        Token { r#type: TokenType::Parenthesis, value: String::from(")")},
        Token { r#type: TokenType::Parenthesis, value: String::from(")")},
    );
    assert_eq!(tokenizer(input), tokens);
}
