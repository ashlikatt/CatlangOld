use crate::tokenization::errors::TokenizationError;
use crate::tokenization::errors::TokenizationError::{InvalidEscapeCharacter, NumOutOfRangeError, UnclosedString, UnexpectedCharacter, UnrecognizedCharacter};
use crate::tokenization::token::{FilePosition, Token};
use crate::tokenization::token_type::TokenType;

mod token;
mod token_type;
mod errors;

/// Converts a string slice into a list of Tokens for future parsing
pub fn tokenize(code: &str) -> Result<Vec<Token>, TokenizationError> {
    let mut column: FilePosition = 0;
    let mut line: FilePosition = 1;
    let mut out = vec![];

    let mut iter = code.chars().peekable();

    // Loop over all chars in code
    while let Some(ch) = iter.next() {
        update_position(ch, &mut column, &mut line);

        // If this character is part of a two-char token
        if let Some(next) = iter.peek() {
            if let Some(token_type) = TokenType::two_char_token(ch, *next) {
                update_position(iter.next().unwrap(), &mut column, &mut line);
                out.push(Token::new_two_char(token_type, column, line));
                continue;
            }
        }

        if ch.is_digit(10) {
            let begin_column = column;
            let begin_line = line;
            let mut value: i64 = ch.to_digit(10).unwrap_or(0) as i64;

            // Int loop
            while let Some(d) = iter.peek() {
                if d.is_digit(10) {
                    let v = iter.next().unwrap();
                    update_position(v, &mut column, &mut line);
                    value = value.checked_mul(10).ok_or_else(|| NumOutOfRangeError { col: column, line })?;
                    value = value.checked_add(v.to_digit(10).unwrap() as i64).ok_or_else(|| NumOutOfRangeError { col: column, line })?;
                } else if *d != '_' {
                    break;
                }
            }

            // Check if we need to do float stuff
            if let Some('.') = iter.peek() {
                iter.next();
                update_position('.', &mut column, &mut line);

                let mut fvalue = value as f64;
                let mut position = 0.1f64;

                // Float loop
                while let Some(d) = iter.peek() {
                    if d.is_digit(10) {
                        let v = iter.next().unwrap();
                        update_position(v, &mut column, &mut line);
                        fvalue += position * v.to_digit(10).unwrap() as f64;
                        position /= 10f64;
                    } else if *d != '_' {
                        break;
                    }
                }

                out.push(Token::new(TokenType::Float(fvalue), begin_column, begin_line, column + 1, line));
            } else {
                out.push(Token::new(TokenType::Int(value), begin_column, begin_line, column + 1, line));
            }

            continue;
        }

        if ch == '.' {
            let begin_column = column;
            let begin_line = line;

            let mut fvalue = 0f64;
            let mut position = 0.1f64;
            let mut changed = false;

            // Float loop
            while let Some(d) = iter.peek() {
                if d.is_digit(10) {
                    let v = iter.next().unwrap();
                    update_position(v, &mut column, &mut line);
                    fvalue += position * v.to_digit(10).unwrap() as f64;
                    position /= 10f64;
                    changed = true;
                } else if *d != '_' {
                    break;
                }
            }

            if !changed {
                return Err(UnexpectedCharacter { ch, col: begin_column, line: begin_line })
            }

            out.push(Token::new(TokenType::Float(fvalue), begin_column, begin_line, column + 1, line));

            continue;
        }

        // If this character is part of a single-char token
        if let Some(token_type) = TokenType::single_char_token(ch) {
            out.push(Token::new_single_char(token_type, column, line));
            continue;
        }

        // Idents
        if TokenType::is_identifier_char(ch, true) {
            let mut s = String::from(ch);
            let begin_column = column;
            let begin_line = line;

            while let Some(ident_ch) = iter.peek() {
                if TokenType::is_identifier_char(*ident_ch, false) {
                    update_position(*ident_ch, &mut column, &mut line);
                    s.push(iter.next().unwrap());
                } else {
                    break;
                }
            }

            out.push(Token::new(TokenType::keyword_token(s.as_str()), begin_column, begin_line, column + 1, line));
            continue;
        }

        // Strings
        if TokenType::is_string_delimiter(ch) {
            let mut s = String::new();
            let begin_column = column;
            let begin_line = line;
            let mut escaped = false;

            loop {
                let str_ch = iter.next().ok_or_else(|| UnclosedString {
                    col: begin_column, line: begin_line
                })?;

                update_position(str_ch, &mut column, &mut line);
                if escaped {
                    s.push(get_escaped_char(str_ch, ch, column, line)?);
                    escaped = false;
                } else {
                    match str_ch {
                        '\\' => escaped = true,
                        x if x == ch => break,
                        x => s.push(x)
                    }
                }
            }

            out.push(Token::new(TokenType::String(s), begin_column, begin_line, column + 1, line));
            continue;
        }

        if !TokenType::is_ignored_char(ch) {
            return Err(UnrecognizedCharacter {
                ch, col: column, line
            });
        }
    }

    Ok(out)
}

/// Updates row and line information for each new character
fn update_position(ch: char, column: &mut FilePosition, line: &mut FilePosition) {
    if ch == '\n' {
        *column = 0;
        *line += 1;
    } else {
        *column += 1;
    }
}

fn get_escaped_char(ch: char, delim: char, col: FilePosition, line: FilePosition) -> Result<char, TokenizationError> {
    match ch {
        'n' => Ok('\n'),
        't' => Ok('\t'),
        'r' => Ok('\r'),
        '\\' => Ok('\\'),
        x if x == delim => Ok(delim),
        _ => Err(InvalidEscapeCharacter { ch, col, line })
    }
}