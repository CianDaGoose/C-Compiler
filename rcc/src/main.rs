use rcc::Lexer;
use rcc::Token;
use std::env;
use std::fs::File;
use std::error::Error;
use std::io::Read;


const SUCCESS: () = ();

fn main() -> Result<(), Box<dyn Error>>{
    let command_line_arguments: Vec<String> = env::args().collect();

    let target_file = command_line_arguments.get(1);

    let mut file = match target_file {
        Some(file) => File::open(file)?,
        None => panic!("No file was provided"),
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut lexer = Lexer::new(&contents);

    while let Some(token) = lexer.next_token() {
        match token {
            Token::Whitespace => continue, // Ignore whitespace tokens
            _ => println!("{:?}", token),
        }
    }

    Ok(SUCCESS)
}
