use crate::tokenization::token::FilePosition;

pub enum TokenizationError {
    UnrecognizedCharacter {
        ch: char,
        col: FilePosition,
        line: FilePosition
    },
    UnexpectedCharacter {
        ch: char,
        col: FilePosition,
        line: FilePosition
    },
    UnclosedString {
        col: FilePosition,
        line: FilePosition
    },
    InvalidEscapeCharacter {
        ch: char,
        col: FilePosition,
        line: FilePosition
    },
    NumOutOfRangeError {
        col: FilePosition,
        line: FilePosition
    }
} impl TokenizationError {
    pub fn get_message(&self) -> String {
        match self {
            TokenizationError::UnrecognizedCharacter {
                ch, col, line
            } => format!("Character '{ch}' is unrecognized.\nAt: line {line}, col {col}"),
            TokenizationError::UnexpectedCharacter {
                ch, col, line
            } => format!("Character '{ch}' is unexpected here.\nAt: line {line}, col {col}"),
            TokenizationError::UnclosedString {
                col, line
            } => format!("Unclosed string.\nAt: line {line}, col {col}"),
            TokenizationError::InvalidEscapeCharacter {
                ch, col, line
            } => format!("Character '{ch}' cannot be escaped with a backslash.\nAt: line {line}, col {col}"),
            TokenizationError::NumOutOfRangeError {
                col, line
            } => format!("Number is out of range.\nAt: line {line}, col {col}"),
        }
    }
}