use crate::{
    error::Error,
    lexer::token::{Token, TokenKind},
    parser::{
        expression::{Expression, ExpressionKind, If},
        Parser,
    },
};

use super::prefix_parselet::PrefixParselet;

pub struct IfParselet;
impl<'a> PrefixParselet<'a> for IfParselet {
    fn parse(
        &self,
        parser: &mut Parser<'a, impl Iterator<Item = Token<'a>>>,
        token: Token<'a>,
    ) -> Result<Expression<'a>, Error<'a>> {
        let condition = parser.parse_expression(0, token.0)?;
        let (then_span, _) = parser.expect(TokenKind::Then, condition.0)?;
        let then_branch = parser.parse_expression(0, then_span)?;
        let mut else_branch = None;
        if let Some((_, TokenKind::Else)) = parser.peek() {
            let (else_span, _) = parser.consume(then_span)?;
            else_branch = Some(Box::new(parser.parse_expression(0, else_span)?));
        }

        Ok((
            token.0,
            ExpressionKind::If(If {
                condition: Box::new(condition),
                then_branch: Box::new(then_branch),
                else_branch,
            }),
        ))
    }
}