//! The Expression struct holds the expressions that are generated by the Parser.
//! The Expression struct maintains the position where the expression was generated and the type of the expression.
//! Using an enum for the values increases the readibility of the code.

use super::expression_kind::ExpressionKind;
use std::fmt::Debug;

pub struct Expression {
    pub kind: ExpressionKind,
    pub pos: usize,
}

impl Expression {
    /// Constructs a new expression with the given value and position.
    ///
    /// # Arguments
    /// `kind` - The value of this Expression.
    /// `pos` - The position where this Expression was created.
    pub fn new(kind: ExpressionKind, pos: usize) -> Expression {
        Expression { kind, pos }
    }
}

impl Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.kind)
    }
}
