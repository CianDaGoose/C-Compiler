use rcc::Lexer;
use rcc::Token;


fn main() {
    let input = "int main() { return 0; }";
    let mut lexer = Lexer::new(input);

    while let Some(token) = lexer.next_token() {
        match token {
            Token::Whitespace => continue, // Ignore whitespace tokens
            _ => println!("{:?}", token),
        }
    }
}
