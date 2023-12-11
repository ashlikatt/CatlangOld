use crate::tokenization::token_type::TokenType::Ident;


/// Represents a type of token.
#[derive(Debug)]
pub enum TokenType {
    // Meta
    String(String),
    Ident(String), IgnoreIdent,
    /*
    Note for numbers: -64 will be tokenized as MINUS, INT. Numbers are signed in the full
    language which is why we're using i64 here instead of u64.
     */
    Int(i64),
    Float(f64),

    // Grouping
    OpenParen, CloseParen, OpenBracket, CloseBracket, OpenBrace, CloseBrace,

    // Structure
    Comma, Period, Semicolon, Colon,

    // Math
    Plus, Minus, Asterisk, Slash, Percent,

    // Assignment
    Assignment,

    // Logic
    GreaterThan, LessThan, GreaterOrEqual, LessOrEqual, Equal,

    // Boolean alg
    True, False, Not, And, Or, Xor,

    // Statement
    If, While, For, Break, Continue, Return,

    // Structure
    Function, Class,

    // Misc.
    Is,
} impl TokenType {
    /// Converts a single char into a TokenType for basic tokens, otherwise None.
    pub fn single_char_token(ch: char) -> Option<TokenType> {
        match ch {
            '(' => Some(TokenType::OpenParen),
            ')' => Some(TokenType::CloseParen),
            '{' => Some(TokenType::OpenBrace),
            '}' => Some(TokenType::CloseBrace),
            '[' => Some(TokenType::CloseBracket),
            ']' => Some(TokenType::OpenBracket),
            ',' => Some(TokenType::Comma),
            '.' => Some(TokenType::Period),
            ';' => Some(TokenType::Semicolon),
            ':' => Some(TokenType::Colon),
            '+' => Some(TokenType::Plus),
            '-' => Some(TokenType::Minus),
            '*' => Some(TokenType::Asterisk),
            '/' => Some(TokenType::Slash),
            '%' => Some(TokenType::Percent),
            '!' => Some(TokenType::Not),
            '<' => Some(TokenType::LessThan),
            '>' => Some(TokenType::GreaterThan),
            _ => None
        }
    }

    pub fn two_char_token(ch1: char, ch2: char) -> Option<TokenType> {
        match (ch1, ch2) {
            ('>', '=') => Some(TokenType::GreaterOrEqual),
            ('<', '=') => Some(TokenType::LessOrEqual),
            ('=', '=') => Some(TokenType::Equal),
            _ => None,
        }
    }

    /// Returns a TokenType from a string, accounting for special identifiers like "true."
    pub fn keyword_token(s: &str) -> TokenType {
        match s {
            "true" => TokenType::True,
            "false" => TokenType::False,
            "and" => TokenType::And,
            "or" => TokenType::Or,
            "xor" => TokenType::Xor,
            "is" => TokenType::Is,
            "_" => TokenType::IgnoreIdent,
            "if" => TokenType::If,
            "while" => TokenType::While,
            "for" => TokenType::For,
            "return" => TokenType::Return,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "fn" => TokenType::Function,
            "class" => TokenType::Class,
            _ => Ident(s.to_string())
        }
    }

    /// Determines if a character is allowed in keywords
    pub fn is_identifier_char(ch: char, beginning: bool) -> bool {
        match ch {
            'a'..='z' | 'A'..='Z' | '_' => true,
            '0'..='9' => !beginning,
            _ => false
        }
    }

    /// Determines if a character is a string delimiter (" or ')
    pub fn is_string_delimiter(ch: char) -> bool {
        ch == '"' || ch == '\''
    }

    /// Returns if a character should be ignored
    pub fn is_ignored_char(ch: char) -> bool {
        ch.is_whitespace()
    }
}