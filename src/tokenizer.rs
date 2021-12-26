#[derive(Debug, PartialEq)]
pub enum TokenType {
    Parenthesis,
    Name,
    Number,
    String,
}

#[derive(Debug)]
pub struct Token {
    pub r#type: TokenType,
    pub value: String,
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

impl Token {
    pub fn new(token_type: TokenType, value: impl Into<String>) -> Token {
        return Token {r#type: token_type, value: value.into()};
    }
}

pub fn tokenizer(input: &str) -> Vec<Token> {
    let bytes = input.as_bytes();

    let mut tokens = Vec::new();
    let mut current = 0;

    while current < bytes.len() {
        let mut char = bytes[current] as char;

        // TokenType::Parenthesis
        if char == '(' {
            tokens.push(Token::new(TokenType::Parenthesis, "("));
            current += 1;
            continue;
        }
        if char == ')' {
            tokens.push(Token::new(TokenType::Parenthesis, ")"));
            current += 1;
            continue;
        }

        // Skip whitespace
        if char.is_whitespace() {
            current += 1;
            continue;
        }

        // TokenType::Number
        if char.is_numeric() {
            let mut value = "".to_string();
            while char.is_numeric() {
                value.push_str(&char.to_string());
                current += 1;
                char = bytes[current] as char;
            }
            tokens.push(Token::new(TokenType::Number, value));
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
            tokens.push(Token::new(TokenType::String, value));
            continue;
        }

        // TokenType::Name
        if char.is_alphabetic() {
            let mut value = "".to_string();
            while char.is_alphabetic() {
                value.push_str(&char.to_string());
                current += 1;
                char = bytes[current] as char;
            }
            tokens.push(Token::new(TokenType::Name, value));
            continue;
        }

        unreachable!();
    }
    return tokens;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tokenizer() {
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
}
