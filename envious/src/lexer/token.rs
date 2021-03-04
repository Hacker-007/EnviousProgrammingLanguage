use std::fmt::Display;

use crate::error::Span;

/// Represents a token that is generated by the `Lexer`.
/// Each token consists of a span (the location information of the token)
/// and the kind of the token.
pub type Token<'a> = (Span<'a>, TokenKind);

/// Enum that details the different types of tokens that can be produced
/// by the `Lexer`. The `TokenKind` should strive to only store types that
/// are small in nature and any other types (i.e. String) should be stored in the
/// `Interner`.
#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Whitespace(char),

    Void,
    Int,
    Float,
    Boolean,
    String,
    IntegerLiteral(i64),
    FloatLiteral(f64),
    BooleanLiteral(bool),
    // The actual value for both the `StringLiteral` and the `Identifier` are
    // stored in the `Interner` to reduce redundency in values. Instead,
    // the id's are stored in the variant.
    StringLiteral(usize),
    Identifier(usize),

    LeftParenthesis,
    RightParenthesis,
    LeftCurlyBrace,
    RightCurlyBrace,
    LeftAngleBracket,
    RightAngleBracket,
    Plus,
    Minus,
    Star,
    Slash,
    PercentSign,
    EqualSign,
    ColonEqualSign,
    ExclamationEqualSign,
    LessThanEqualSign,
    GreaterThanEqualSign,
    Comma,
    Colon,
    ColonColon,

    Not,
    Or,
    And,
    Let,
    If,
    Then,
    Else,
    While,
    Define,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TokenKind::Whitespace(ch) => write!(f, "{}", ch),
            TokenKind::Void => write!(f, "Void"),
            TokenKind::Int => write!(f, "Int"),
            TokenKind::Float => write!(f, "Float"),
            TokenKind::Boolean => write!(f, "Boolean"),
            TokenKind::String => write!(f, "String"),
            TokenKind::IntegerLiteral(_) => write!(f, "integer literal"),
            TokenKind::FloatLiteral(_) => write!(f, "float literal"),
            TokenKind::BooleanLiteral(_) => write!(f, "boolean literal"),
            TokenKind::StringLiteral(_) => write!(f, "string literal"),
            TokenKind::Identifier(_) => write!(f, "identifier"),
            TokenKind::LeftParenthesis => write!(f, "("),
            TokenKind::RightParenthesis => write!(f, ")"),
            TokenKind::LeftCurlyBrace => write!(f, "{{"),
            TokenKind::RightCurlyBrace => write!(f, "}}"),
            TokenKind::LeftAngleBracket => write!(f, "<"),
            TokenKind::RightAngleBracket => write!(f, ">"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::PercentSign => write!(f, "%"),
            TokenKind::EqualSign => write!(f, "="),
            TokenKind::ColonEqualSign => write!(f, ":="),
            TokenKind::ExclamationEqualSign => write!(f, "!="),
            TokenKind::LessThanEqualSign => write!(f, "<="),
            TokenKind::GreaterThanEqualSign => write!(f, ">="),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::ColonColon => write!(f, "::"),
            TokenKind::Not => write!(f, "not"),
            TokenKind::Or => write!(f, "or"),
            TokenKind::And => write!(f, "and"),
            TokenKind::Let => write!(f, "let"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Then => write!(f, "then"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::While => write!(f, "while"),
            TokenKind::Define => write!(f, "define"),
        }
    }
}
