use std::iter::Peekable;

use gdtk_lexer::Token;

/// A wrapper around token iterator to add conditional token emitting.
#[derive(Debug)]
pub struct Parser<I> {
    pub iter: I,
    pub is_inside_parens: bool,
}

impl<'a, I> Parser<Peekable<I>>
where
    I: Iterator<Item = Token<'a>>,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter: iter.peekable(),
            is_inside_parens: false,
        }
    }

    pub fn peek(&mut self) -> Option<&Token<'a>> {
        self.skip_blanks();
        self.iter.peek()
    }

    /// Invoke a function inside the context of a parenthesized expression.
    pub fn with_parens_ctx<F, R>(&mut self, val: bool, mut f: F) -> R
    where
        F: FnMut(&mut Self) -> R,
    {
        let previous = self.is_inside_parens;

        self.is_inside_parens = val;

        let result = f(self);

        self.is_inside_parens = previous;

        result
    }

    fn skip_blanks(&mut self) {
        if self.is_inside_parens {
            while self
                .iter
                .peek()
                .is_some_and(|t| t.kind.is_newline() || t.kind.is_indent() || t.kind.is_dedent())
            {
                self.iter.next();
            }
        }
    }
}

impl<'a, I> Iterator for Parser<Peekable<I>>
where
    I: Iterator<Item = Token<'a>>,
{
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_blanks();
        self.iter.next()
    }
}
