use logos::{Logos, Lexer};

fn template_literal<'a>(lex: &mut Lexer<Token>) -> () {
    
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n]")]
pub enum Token {
    #[regex(r"\d[_\d]*")] // Integer
    #[regex(r"\.\d[_\d]*")] // Decimal
    #[regex(r"\d[_\d]*\.[_\d]*")] // Float
    #[regex(r"\d[_\d]*\.[_\d]*[eE][\-\+]?[_\d]*\d")] // Scientific notation
    #[regex(r"0_*[xX][_\da-fA-F]*[\da-fA-F][_\da-fA-F]*")] // Hexadecimal
    #[regex(r"0_*[bB][_01]*[01][_01]*")] // Binary
    NumberLiteral,

    #[regex(r#""([^"\\]|\\.)*""#)]
    StringLiteral,

    #[token("`", template_literal)]
    TemplateLiteral,

    #[token("if")]
    If,

    #[token("else")]
    Else,

    #[token("while")]
    While,

    #[token("for")]
    For,

    #[token("in")]
    In,

    #[token("break")]
    Break,

    #[token("continue")]
    Continue,

    #[token("return")]
    Return,

    #[token("function")]
    Function,

    #[token("local")]
    Local,

    #[token("nil")]
    Nil,

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("and")]
    And,

    #[token("or")]
    Or,

    #[token("not")]
    Not,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Asterisk,

    #[token("/")]
    Slash,

    #[token("%")]
    Percent,

    #[token("^")]
    Caret,

    #[token("#")]
    Hash,

    #[token("==")]
    DoubleEquals,

    #[token("~=")]
    TildeEquals,

    #[token("<")]
    RightAngleBracket,

    #[token(">")]
    LeftAngleBracket,

    #[token("<=")]
    RightAngleBracketEquals,

    #[token(">=")]
    LeftAngleBracketEquals,

    #[token("=")]
    Equals,

    #[token("(")]
    LeftParenthesis,

    #[token(")")]
    RightParenthesis,

    #[token("{")]
    LeftCurlyBracket,

    #[token("}")]
    RightCurlyBracket,

    #[token("[")]
    LeftSquareBracket,

    #[token("]")]
    RightSquareBracket,

    #[token(",")]
    Comma,

    #[token(".")]
    Dot,

    #[token(":")]
    Colon,

    #[token(";")]
    Semicolon,

    #[token("...")]
    Ellipsis, // BusyCityGuy reference
}