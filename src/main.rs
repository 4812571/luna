use logos::{Logos, Lexer};

#[derive(Logos, Debug, PartialEq)]
enum NestedLanguageString {
    StartBrace,
    EndBrace,
}