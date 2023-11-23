pub mod callbacks;
pub mod error;
pub mod token;
#[cfg(test)]
mod tests;

use error::{WithSpan, IntoDiag};
use logos::Logos;
use gdtk_diag::Diagnostic;
use token::Token;

pub type Lexeme<'a> = (Token<'a>, logos::Span);
pub type LexOutput<'a> = (Vec<Lexeme<'a>>, Vec<Diagnostic>);

pub fn lex(input: &str) -> LexOutput {
    preprocess(Token::lexer(input))
}

/// Arranges results by their span.
fn preprocess<'a>(
    lexer: logos::Lexer<'a, Token<'a>>,
) ->  LexOutput {
    let mut tokens: Vec<Lexeme> = vec![];
    let mut diags: Vec<Diagnostic> = vec![];

    for (result, span) in lexer.spanned() {
        match result {
            Ok(token) => tokens.push(token.with_span(span)),
            Err(err) => diags.push(err.with_span(span).into_diag()),
        }
    }

    (tokens, diags)
}
