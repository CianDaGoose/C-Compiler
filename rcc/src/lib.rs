use std::str::Chars;

/// Token types
#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Number(String),
    Symbol(char),
    Whitespace,
    Unknown(char),
}

/// Lexer struct
pub struct Lexer<'a> {
    input: Chars<'a>,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let current_char = chars.next();
        Lexer {
            input: chars,
            current_char,
        }
    }

    /// Advance to the next character
    fn advance(&mut self) {
        self.current_char = self.input.next();
    }

    /// Peek at the current character
    fn peek(&self) -> Option<char> {
        self.current_char
    }

    /// Skip whitespace
    #[allow(dead_code)]
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Tokenize the input
    pub fn next_token(&mut self) -> Option<Token> {
        while let Some(c) = self.peek() {
            match c {
                // Handle whitespace
                c if c.is_whitespace() => {
                    self.advance();
                    return Some(Token::Whitespace);
                }
                // Handle numbers
                c if c.is_ascii_digit() => return Some(self.lex_number()),
                // Handle keywords and identifiers
                c if c.is_ascii_alphabetic() || c == '_' => return Some(self.lex_identifier_or_keyword()),
                // Handle symbols
                c if is_symbol(c) => {
                    self.advance();
                    return Some(Token::Symbol(c));
                }
                // Handle unknown characters
                _ => {
                    self.advance();
                    return Some(Token::Unknown(c));
                }
            }
        }
        None // End of input
    }

    /// Lex numbers
    fn lex_number(&mut self) -> Token {
        let mut num = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                num.push(c);
                self.advance();
            } else {
                break;
            }
        }
        Token::Number(num)
    }

    /// Lex keywords or identifiers
    fn lex_identifier_or_keyword(&mut self) -> Token {
        let mut id = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                id.push(c);
                self.advance();
            } else {
                break;
            }
        }
        if is_keyword(&id) {
            Token::Keyword(id)
        } else {
            Token::Identifier(id)
        }
    }
}


/// Check if a string is a keyword
fn is_keyword(word: &str) -> bool {
    matches!(word, "if" | "else" | "for" | "while" | "int" | "return" | "void")
}

/// Check if a character is a symbol
fn is_symbol(c: char) -> bool {
    matches!(c, '+' | '-' | '*' | '/' | '=' | '<' | '>' | ';' | ',' | '(' | ')' | '{' | '}')
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn is_keyword_valid() {
        assert_eq!(is_keyword("return"), true, "return should be made a keyword if failed.");
        assert_eq!(is_keyword("hello"), false, "hello should NOT be a keyword.");
    }

    #[test]
    fn is_symbol_valid() {
        assert_eq!(is_symbol('|'), false, "Pipe is not yet supported as a symbol, should be removed.");
        assert_eq!(is_symbol(';'), true, "semi-colon should be made a valid symbol.");
    }

    #[test]
    fn lexer_basic_operations() -> Result<(), Box<dyn std::error::Error>> {
        let contents = "int main(void) { return 0; }";

        let mut lexer = Lexer::new(contents);
        let mut tokens = Vec::new();

        while let Some(token) = lexer.next_token() {
            match token {
                Token::Whitespace => continue, // Skip whitespace tokens
                _ => tokens.push(token),      // Collect other tokens
            }
        }

        let expected_tokens = vec![
            Token::Keyword("int".to_string()),
            Token::Identifier("main".to_string()),
            Token::Symbol('('),
            Token::Keyword("void".to_string()),
            Token::Symbol(')'),
            Token::Symbol('{'),
            Token::Keyword("return".to_string()),
            Token::Number("0".to_string()),
            Token::Symbol(';'),
            Token::Symbol('}'),
        ];

        assert_eq!(tokens, expected_tokens, "The lexer did not produce the expected tokens.");
        Ok(())
    }

}
