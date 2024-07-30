use core::fmt;
use std::env;
use std::fs;
use std::io::{self, Write};

use std::process;

fn main() {
    

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return ;     
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

            let _ =tokenize(&file_contents);        
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return ;
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
    Equal,
    EqualEqual,
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
            Token::Equal => write!(f, "EQUAL = null"),
            Token::EqualEqual => write!(f, "EQUAL_EQUAL == null"),

        }
    }
}


fn tokenize(lexeme: &str) -> Result<(), i32>{

    let mut status = 0;

    let line = 1;
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
            '=' => {
                
                if lexeme.chars().count() > 1 && lexeme.chars().nth(1).unwrap() == '=' {
                    println!("{}", Token::EqualEqual);
                }
                else {
                    
                    println!("{}", Token::Equal);
                }
            },
            _ =>{
                eprintln!("[line {}] Error: Unexpected character: {}",line, f);
                status = 65;
                
            }
        };
        // line+=1;
    }
    println!("{}",Token::EOF);

    process::exit(status);
}
