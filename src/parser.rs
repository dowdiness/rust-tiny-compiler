use crate::tokenizer::*;

#[derive(Debug)]
enum AstType {
    Program,
    NumberLiteral,
    StringLiteral,
    CallExpression,
}

#[derive(Debug)]
struct AstNode {
    r#type: AstType,
    name: Option<String>,
    value: Option<String>,
    params: Option<Vec<AstNode>>,
}

#[derive(Debug)]
pub struct Ast {
    r#type: AstType,
    body: Vec<AstNode>,
}

pub fn parser(tokens: Vec<Token>) -> Ast {
    let mut current = 0;
    let mut ast: Ast = Ast {
        r#type: AstType::Program,
        body: Vec::new(),
    };

    while current < tokens.len() {
        let (token, curr) = walk(&tokens, current);
        current = curr;
        ast.body.push(token);
    }

    return ast;
}

fn walk(tokens: &[Token], current: usize) -> (AstNode, usize) {
    let mut current = current;
    let mut token = &tokens[current];
    println!("Begginng of walk: {:?}, Current {:?}", token, current);

    if token.r#type == TokenType::Number {
        current += 1;
        return (
            AstNode {
                r#type: AstType::NumberLiteral,
                name: None,
                value: Some(token.value.clone()),
                params: None,
            },
            current,
        );
    }

    if token.r#type == TokenType::String {
        current += 1;
        return (
            AstNode {
                r#type: AstType::StringLiteral,
                name: None,
                value: Some(token.value.clone()),
                params: None,
            },
            current,
        );
    }

    if token.r#type == TokenType::Parenthesis && token.value == "(" {
        // Skip Parenthesis type Token to Name type Token
        current += 1;
        token = &tokens[current];
        println!(
            "Begginng of CallExpressions: {:?}, Current {:?}",
            token, current
        );

        let mut node = AstNode {
            r#type: AstType::CallExpression,
            name: Some(token.value.clone()),
            value: None,
            params: Some(Vec::new()),
        };

        // Skip Name type Token
        current += 1;
        token = &tokens[current];

        loop {
            if token.r#type == TokenType::Parenthesis && token.value == ")" {
                break;
            }

            let offset = walk(tokens, current);
            node.params.as_mut().unwrap().push(offset.0);
            current = offset.1;
            token = &tokens[current];
        }
        println!(
            "End of CallExpression: {:?}, Current {:?}",
            &tokens[current], current
        );

        // Skip closing parenthesis
        current += 1;
        return (node, current);
    }

    println!("unreachable: {:?}, Current {:?}", token, current);

    unreachable!();
}
