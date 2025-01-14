pub mod comps {
    pub mod lexer;
}

#[cfg(test)]
mod lexer_unit_test {
    use super::comps::lexer::Lexer;
    use super::comps::lexer::Token;
    use super::comps::lexer::is_keyword;
    use super::comps::lexer::is_symbol;

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
