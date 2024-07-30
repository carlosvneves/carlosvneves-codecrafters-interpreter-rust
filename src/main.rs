use core::fmt;
use std::env;
use std::fs;
use std::io::{self, Write};

use std::process;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return Ok(());     
    }

    let command = &args[1];
    let filename = &args[2];

    

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            match tokenize(&file_contents){
                Ok(_) => {
                    process::exit(0);
                },
                Err(e) => {
                    writeln!(io::stderr(), "Failed to tokenize: {}", e).unwrap();
                    process::exit(65);
                }
            }

        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return Ok(());
        }
    }
}

// <token_type> <lexeme> <literal>
//, , ., -, +, ;, *. /
enum Token{
    EOF,
    RightParen,     
    LeftParen,
    RightBrace,
    LeftBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    Slash,


    // Dot,
    // Comma,
    // Plus,
    // Minus,
    // Star,
    // Slash,
    // Bang,
    // BangEqual,
    // Equal,
    // EqualEqual,
    // Greater,
    // GreaterEqual,
    // Less,
    // LessEqual,
    // Identifier(String),
    // StringLiteral(String),
    // NumberLiteral(f64),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::EOF => write!(f, "EOF  null"),
            Token::RightParen => write!(f, "RIGHT_PAREN ) null"),
            Token::LeftParen => write!(f, "LEFT_PAREN ( null"),
            Token::RightBrace => write!(f, "RIGHT_BRACE }} null"),
            Token::LeftBrace => write!(f, "LEFT_BRACE {{ null"),
            Token::Comma => write!(f, "COMMA , null"),
            Token::Dot => write!(f, "DOT . null"),
            Token::Minus => write!(f, "MINUS - null"),
            Token::Plus => write!(f, "PLUS + null"),
            Token::Semicolon => write!(f, "SEMICOLON ; null"),
            Token::Star => write!(f, "STAR * null"),
            Token::Slash => write!(f, "SLASH / null"),

        }
    }
}


fn tokenize(lexeme: &str) -> Result<(), i32>{
    let mut line = 1;

    for f in lexeme.chars() {
        match f {
            '(' => {
                println!("{}",Token::LeftParen);
            },
            ')' => {
                println!("{}",Token::RightParen);
            },
            '{' => {
                println!("{}", Token::LeftBrace);
            },
            '}' =>{
                println!("{}", Token::RightBrace);

            },
            ',' => {
                println!("{}", Token::Comma);
            },
            '.' =>{
                println!("{}", Token::Dot);
            },
            '-' =>{
                println!("{}", Token::Minus);
            },
            '+' =>{
                println!("{}", Token::Plus);
            },
            ';' =>{
                println!("{}", Token::Semicolon);
            },
            '*' =>{
                println!("{}", Token::Star);
            },
            '/' =>{
                println!("{}", Token::Slash);
            },
            _ =>{
                eprintln!("[line {}] Error: Unexpected character: {}",line, f);
                eprintln!("{}", Token::EOF);
                process::exit(65);
                
                
            }
        };
        line += 1;
    }
    println!("{}",Token::EOF);

    process::exit(0);
}
