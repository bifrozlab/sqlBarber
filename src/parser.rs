use tree_sitter::{Tree, TreeCursor, Node};
use crate::ast::{Expr, Statement};
use crate::tsnode;
use crate::tsnode::ts_kind_hash;

#[derive(Debug, Clone)]
pub enum ParseError {
    TreeSitterError(String),
    SyntaxTreeError(String),
}

impl ParseError {
    fn new_syntax_tree_error<T>(message: &str, node: &Node) -> Result<T, ParseError>{
        Err(Self::SyntaxTreeError(
            format!(
                "Error in TreeSitter Syntax Tree: {} {}[L{},{}]",
                message, node.kind(), node.start_position(), node.end_position()
            )
        ))
    }
}

pub fn parse_tree_sitter(source_tree: Tree) -> Result<Vec<Statement>, ParseError> {
    let multistmt: Node = source_tree.root_node();
    let cursor: TreeCursor = multistmt.walk();

    multistmt
        .named_children(&mut cursor)
        .map(|stmt| match ts_kind_hash(stmt.kind()) {
            tsnode::SELECT => parse_tree_sitter_select(&stmt),
            tsnode::SELECT_IN_PARENS => {
                let mut child = stmt.named_child(0);
                while (child?.kind() == tsnode::SELECT_IN_PARENS) {
                    child = child.named_child(0);
                }

                match child {
                    Some(node) if node.kind() == tsnode::SELECT => parse_tree_sitter_select(&node),
                    Some(node) => ParseError::new_syntax_tree_error("Expected SELECT but found", &node),
                    None => ParseError::new_syntax_tree_error("No child node in", &stmt),
                }
            },
        })
}

/// Parse Tree Sitter "select" node to Select
fn parse_tree_sitter_select(select: &Node) -> Result<Select, ParseError> {

}
 
/// Parses Tree Sitter "expr" node to Expr
fn parse_tree_sitter_expression(source: &[u8], node: &Node) -> Result<Expr, ParseError> {
    let mut cursor: TreeCursor;

    match ts_kind_hash(node.kind()) {
        tsnode::UNARY_EXPR => {
            if node.child_count() != 2 {
                return ParseError::new_syntax_tree_error("Invalid UNARY_EXPR", &node);
            }

            cursor = node.walk();
            cursor.goto_first_child();
            let operator_str = cursor.node().utf8_text(source).unwrap().to_uppercase();

            cursor.goto_next_sibling();
            let sub_expr = Box::new(parse_tree_sitter_expression(source, &cursor.node())?);

            match &operator_str[..] {
                "-" => Ok(Expr::Minus(sub_expr)),
                "+" => Ok(Expr::Plus(sub_expr)),
                "~" => Ok(Expr::BitwiseNot(sub_expr)),
                "NOT" => Ok(Expr::Not(sub_expr)),
                _ => ParseError::new_syntax_tree_error("Invalid UNARY_EXPR Operator", &node),
            }
        },
        tsnode::BINARY_EXPR => {
            if node.child_count() != 3 {
                return ParseError::new_syntax_tree_error("Invalid BINARY_EXPR", &node);
            }

            cursor = node.walk();
            cursor.goto_first_child();
            let left = Box::new(parse_tree_sitter_expression(source, &cursor.node())?);

            cursor.goto_next_sibling();
            let operator_str = cursor.node().utf8_text(source).unwrap().to_uppercase();

            cursor.goto_next_sibling();
            let right = Box::new(parse_tree_sitter_expression(source, &cursor.node())?);

            match &operator_str[..] {
                "+" => Ok(Expr::Add(left, right)),
                "-" => Ok(Expr::Subtract(left, right)),
                "*" => Ok(Expr::Multiply(left, right)),
                "/" => Ok(Expr::Divide(left, right)),
                "%" => Ok(Expr::Modulo(left, right)),
                "<<" => Ok(Expr::LeftShift(left, right)),
                ">>" => Ok(Expr::RightShift(left, right)),
                "&" => Ok(Expr::BitwiseAnd(left, right)),
                "|" => Ok(Expr::BitwiseOr(left, right)),
                "^" => Ok(Expr::BitwiseXor(left, right)),
                "=" => Ok(Expr::Equal(left, right)),
                "<>" | "!=" => Ok(Expr::NotEqual(left, right)),
                ">" => Ok(Expr::Greater(left, right)),
                ">=" => Ok(Expr::GreaterEqual(left, right)),
                "<" => Ok(Expr::Lesser(left, right)),
                "<=" => Ok(Expr::LesserEqual(left, right)),
                "AND" => Ok(Expr::And(left, right)),
                "OR" => Ok(Expr::Or(left, right)),
                _ => ParseError::new_syntax_tree_error("Invalid BINARY_EXPR Operator", &node),
            }
        },
        _ => ParseError::new_syntax_tree_error("Invalid EXPR", &node),
    }
}
