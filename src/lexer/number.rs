use regex::Regex;

use super::{starting_regex, LexerUnit, Token};

pub struct NumberLexer {
    integer: Regex,
    decimal: Regex,
    float: Regex,
    scientific: Regex,
    hex: Regex,
    bin: Regex,
}

impl NumberLexer {
    pub fn new() -> Self {
        // Digits
        let digit = r"\d";
        let hex_digit = r"{digit}a-fA-F";
        let bin_digit = r"01";

        // Prefixes
        let hex_prefix = r"0_*[xX]";
        let bin_prefix = r"0_*[bB]";

        // Examples: 1, 12, 1_0_0, 1_000_000
        let integer = format!(r"[{digit}][_{digit}]*");

        // Examples: .1, .12, .1_0_0, .1_000_000
        let decimal = format!(r"\.[{digit}][_{digit}]*");

        // Examples: 1.1, 12.12_, 1._, 12.12, 1_._1_
        let float = format!(r"[{digit}][_{digit}]*\.[_{digit}]*");
    
        // Examples: 1e1, 12e12, 1_0_0e1_0_0, 12.32_e1_0_0, 1e1_0_0
        let scientific = format!(r"{float}[eE][\-\+]?[_{digit}]*[{digit}]");

        // Examples: 0x1, 0x12, 0x1_0_0, 0x1_000_000
        let hex = format!(r"{hex_prefix}[_{hex_digit}]*[{hex_digit}][_{hex_digit}]*");

        // Examples: 0b1, 0b10_, 0b_1_0_0, 0b_1_001_010__
        let bin = format!(r"{bin_prefix}[_{bin_digit}]*[{bin_digit}][_{bin_digit}]*");
        
        Self {
            integer: starting_regex(&integer),
            decimal: starting_regex(&decimal),
            float: starting_regex(&float),
            scientific: starting_regex(&scientific),
            hex: starting_regex(&hex),
            bin: starting_regex(&bin),
        }
    }
}

impl LexerUnit for NumberLexer {
    fn lex(&self, s: &str) -> Option<(usize, Token)> {
        self.integer.find(s)
            .or_else(|| self.decimal.find(s))
            .or_else(|| self.float.find(s))
            .or_else(|| self.scientific.find(s))
            .or_else(|| self.hex.find(s))
            .or_else(|| self.bin.find(s))
            .map(|m| (m.end(), Token::NumberLiteral(m.as_str().to_string())))
    }
}