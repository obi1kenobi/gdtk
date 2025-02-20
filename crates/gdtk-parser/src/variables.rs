use gdtk_ast::poor::{ASTVariable, ASTVariableKind};
use gdtk_lexer::{Token, TokenKind};

use crate::expressions::parse_expr;
use crate::misc::parse_type;
use crate::utils::expect;
use crate::Parser;

/// Parses variable body, i.e. any variable without preceding keywords.
pub fn parse_variable_body<'a>(
    iter: &mut Parser<impl Iterator<Item = Token<'a>>>,
    kind: ASTVariableKind,
) -> ASTVariable<'a> {
    let identifier = expect!(iter, TokenKind::Identifier(s), s);
    let mut typehint = None;
    let mut infer_type = false;
    let mut value = None;

    // Possible cases:
    // [var] ident
    // [var] ident = val
    // [var] ident := val
    // [var] ident: type = val
    // [var] ident: type

    match iter.peek().map(|t| &t.kind) {
        Some(TokenKind::Colon) => {
            iter.next();

            match iter.peek().expect("unexpected EOF").kind {
                TokenKind::Assignment => {
                    iter.next();
                    infer_type = true;
                    value = Some(parse_expr(iter));
                }
                _ => {
                    typehint = Some(parse_type(iter));

                    if let Some(TokenKind::Assignment) = iter.peek().map(|t| &t.kind) {
                        iter.next();

                        value = Some(parse_expr(iter));
                    }
                }
            };
        }
        Some(TokenKind::Assignment) => {
            iter.next();
            value = Some(parse_expr(iter))
        }
        _ => (),
    }

    ASTVariable {
        identifier,
        infer_type,
        typehint,
        value,
        kind,
    }
}

#[cfg(test)]
mod tests {
    use gdtk_ast::poor::*;

    use crate::test_utils::create_parser;
    use crate::variables::parse_variable_body;

    #[test]
    fn test_variable_empty() {
        let mut parser = create_parser("ident");
        let result = parse_variable_body(&mut parser, ASTVariableKind::Regular);
        let expected = ASTVariable {
            identifier: "ident",
            infer_type: false,
            typehint: None,
            value: None,
            kind: ASTVariableKind::Regular,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_variable_with_type() {
        let mut parser = create_parser("ident: type");
        let result = parse_variable_body(&mut parser, ASTVariableKind::Regular);
        let expected = ASTVariable {
            identifier: "ident",
            infer_type: false,
            typehint: Some(ASTValue::Identifier("type")),
            value: None,
            kind: ASTVariableKind::Regular,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_variable_with_value() {
        let mut parser = create_parser("ident = 0");
        let result = parse_variable_body(&mut parser, ASTVariableKind::Regular);
        let expected = ASTVariable {
            identifier: "ident",
            infer_type: false,
            typehint: None,
            value: Some(ASTValue::Number(0)),
            kind: ASTVariableKind::Regular,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_variable_with_type_inference_and_value() {
        let mut parser = create_parser("ident := 0");
        let result = parse_variable_body(&mut parser, ASTVariableKind::Regular);
        let expected = ASTVariable {
            identifier: "ident",
            infer_type: true,
            typehint: None,
            value: Some(ASTValue::Number(0)),
            kind: ASTVariableKind::Regular,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_variable_with_type_and_value() {
        let mut parser = create_parser("ident: type = 0");
        let result = parse_variable_body(&mut parser, ASTVariableKind::Regular);
        let expected = ASTVariable {
            identifier: "ident",
            infer_type: false,
            typehint: Some(ASTValue::Identifier("type")),
            value: Some(ASTValue::Number(0)),
            kind: ASTVariableKind::Regular,
        };

        assert_eq!(result, expected);
    }
}
