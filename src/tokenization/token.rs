use crate::tokenization::token_type::TokenType;

pub type FilePosition = u64;

/// Represents a token in the program
#[derive(Debug)]
pub struct Token {
    pub token: TokenType,
    pub column: FilePosition,
    pub line: FilePosition,
    pub end_column: FilePosition,
    pub end_line: FilePosition
} impl Token {
    pub fn new_single_char(token: TokenType, column: FilePosition, line:FilePosition) -> Self {
        Self {
            token,
            column,
            line,
            end_column: column + 1,
            end_line: line,
        }
    }

    pub fn new_two_char(token: TokenType, column: FilePosition, line:FilePosition) -> Self {
        Self {
            token,
            column,
            line,
            end_column: column + 2,
            end_line: line,
        }
    }

    pub fn new(token: TokenType, column: FilePosition, line: FilePosition, end_column: FilePosition, end_line: FilePosition) -> Token {
        Token {token, column, line, end_line, end_column}
    }
}