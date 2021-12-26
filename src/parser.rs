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

  if token.r#type == TokenType::Number {
      current += 1;
      return (
        AstNode {
          r#type: AstType::NumberLiteral,
          name: None,
          value: Some(token.value.clone()),
          params: None,
        },
        current)
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
        current)
  }

  if token.r#type == TokenType::Parenthesis && token.value == "(" {
      current += 1;
      token = &tokens[current];

      let mut node = AstNode {
          r#type: AstType::CallExpression,
          name: Some(token.value.clone()),
          value: None,
          params: Some(Vec::new()),
      };

      current += 1;
      token = tokens.get(current).unwrap();

      while token.r#type != TokenType::Parenthesis ||
      (token.r#type == TokenType::Parenthesis && token.value != ")") {
        node.params.as_mut().unwrap().push(walk(tokens, current).0);
        current += 1;
        token = tokens.get(current).unwrap();
      }

      current += 1;
      return (node, current);
  }

  unreachable!();
}
