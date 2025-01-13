# C Compiler in Rust  

Welcome to the **C Compiler** project, a lightweight and efficient C language compiler built from scratch using Rust. This project is designed to parse, analyze, and compile C programs into executable machine code.  

## Features  
- **Lexical Analysis**: Tokenizes the source code into meaningful symbols.  
- **Parsing**: Constructs an Abstract Syntax Tree (AST) from the tokens.  
- **Semantic Analysis**: Validates types, scopes, and other C language semantics.  
- **Code Generation**: Outputs efficient machine code or assembly.  
- **Error Reporting**: Provides detailed error messages for syntax and semantic issues.  

## Getting Started  

### Prerequisites  
To build and run this compiler, ensure you have the following installed:  
- **Rust** (latest stable version)  
  - Install from [Rust's official website](https://www.rust-lang.org/).  
- **LLVM** (optional, if used for backend code generation)  

### Installation  
Clone the repository:  
```bash  
git clone https://github.com/yourusername/c-compiler.git  
cd c-compiler  
```  

### Building the Project  
Compile the project using Cargo:  
```bash  
cargo build --release  
```  

### Running the Compiler  
To compile a C file:  
```bash  
cargo run -- <source_file.c> -o <output_file>  
```  
Replace `<source_file.c>` with the path to your C source file and `<output_file>` with the desired name of the compiled binary.  

### Example  
```bash  
cargo run -- examples/hello.c -o hello  
./hello  
```  

## Project Structure  
- `src/`: Contains the Rust source code for the compiler.  
  - `lexer.rs`: Handles lexical analysis.  
  - `parser.rs`: Constructs the AST from tokens.  
  - `semantic.rs`: Performs semantic checks.  
  - `codegen.rs`: Generates machine code or assembly.  
- `examples/`: Contains sample C programs for testing the compiler.  
- `tests/`: Includes unit and integration tests for the compiler components.  

## Contributing  
Contributions are welcome! Please follow these steps:  
1. Fork the repository.  
2. Create a new branch (`git checkout -b feature-name`).  
3. Commit your changes (`git commit -m "Add feature-name"`).  
4. Push to the branch (`git push origin feature-name`).  
5. Open a pull request.  

## License  
This project is licensed under the [MIT License](LICENSE).  

## Acknowledgments  
- [Rust Programming Language](https://www.rust-lang.org/)  
- [LLVM Project](https://llvm.org/) (if applicable)  
- Community contributors and testers  
