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
    params: Option<Vec<AstNode>>
}

#[derive(Debug)]
pub struct Ast {
    r#type: AstType,
    body: Vec<AstNode>,
}

pub fn parser(tokens: &mut Vec<Token>) -> Ast {
    let mut current = 0;
    fn walk(tokens: &mut Vec<Token>, current: &mut usize) -> AstNode{
        let mut token = tokens.get(*current).unwrap();

        if token.r#type == TokenType::Number {
            *current += 1;
            return AstNode {
                r#type: AstType::NumberLiteral,
                name: None,
                value: Some(token.value.clone()),
                params: None,
            }
        }

        if token.r#type == TokenType::String {
            *current += 1;
            return AstNode {
                r#type: AstType::StringLiteral,
                name: None,
                value: Some(token.value.clone()),
                params: None,
            }
        }

        // if token.r#type == TokenType::Parenthesis && token.value == "(" {
        //     *current += 1;
        //     token = tokens.get(*current).unwrap();

        //     let mut node = AstNode {
        //         r#type: AstType::CallExpression,
        //         name: Some(token.value.clone()),
        //         value: None,
        //         params: Some(Vec::new()),
        //     };

        //     *current += 1;
        //     token = tokens.get(*current).unwrap();

        //     while token.r#type != TokenType::Parenthesis || token.r#type == TokenType::Parenthesis && token.value != ")" {
        //         node.params.unwrap().push(walk(tokens, &mut current));
        //     }
        // }

        unreachable!();
    };
    let mut ast: Ast = Ast {
        r#type: AstType::Program,
        body: Vec::new(),
    };

    while current < tokens.len() {
        ast.body.push(walk(tokens, &mut current))
    }

    return ast;
}
