use crate::{
    error::Error,
    lexer::token::{Token, TokenKind},
    parser::{
        expression::{Expression, ExpressionKind},
        Parser,
    },
};

use super::prefix_parselet::PrefixParselet;

macro_rules! get {
    ($token: ident, $pattern: pat, $expression: expr) => {
        if let $pattern = $token.1 {
            $expression
        } else {
            unreachable!()
        };
    };
}

pub struct FloatParselet;
impl<'a> PrefixParselet<'a> for FloatParselet {
    fn parse(
        &self,
        _: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
        token: Token<'a>,
    ) -> Result<Expression<'a>, Error<'a>> {
        let value = get!(token, TokenKind::FloatLiteral(value), value);
        Ok((token.0, ExpressionKind::Float(value)))
    }
}
