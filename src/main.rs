use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const MEMORY_SIZE: usize = 30000;

#[derive(PartialEq,Eq)]
enum Token {
    IncDP,
    DecDP,
    IncMemory,
    DecMemory,
    Put,
    Read,
    While,
    EndWhile
}

fn calculate_jumptable(program: &Vec<Token>) -> Vec<usize> {
    let mut jumptable: Vec<usize> = vec![0; program.len()];
    let mut pc: usize = 0;

    while pc < program.len() {
        let ref instruction: Token = program[pc];
        if instruction == &Token::While {
            let mut bracket_nesting = 1;
            let mut seek = pc;

            while bracket_nesting > 0 && seek < program.len() - 1 {
                seek += 1;
                if program[seek] == Token::EndWhile {
                    bracket_nesting -= 1;
                } else if program[seek] == Token::While {
                    bracket_nesting += 1;
                }
            }

            if bracket_nesting > 0 {
                panic!("Unmatched [ at pc={}", pc);
            } else {
                jumptable[pc] = seek;
                jumptable[seek] = pc;
            }
        }
        pc += 1
    }
    jumptable
}

fn simpleinterp(program: &Vec<Token>) {
    let mut memory: Vec<u8> = vec![0; MEMORY_SIZE];
    let jumptable = calculate_jumptable(&program);
    let mut pc: usize = 0;
    let mut dataptr: usize = 0;
    while pc < program.len() {
        let ref instruction: Token = program[pc];
        match instruction {
            &Token::IncDP  => dataptr = dataptr.wrapping_add(1),
            &Token::DecDP  => dataptr = dataptr.wrapping_sub(1),
            &Token::IncMemory => memory[dataptr] = memory[dataptr].wrapping_add(1),
            &Token::DecMemory => memory[dataptr] = memory[dataptr].wrapping_sub(1),
            &Token::Read => panic!(", not implemented"),
            &Token::Put => print!("{}", memory[dataptr] as char),
            &Token::While => {
                if memory[dataptr] == 0 {
                    pc = jumptable[pc];
                }
            },
            &Token::EndWhile => {
                if memory[dataptr] != 0 {
                    pc = jumptable[pc];
                }
            }
        }
        pc += 1;
    }
}

fn read_file(filename: &str) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    Ok(content)
}

fn parse(input: &str) -> Vec<Token> {
    let mut parsed_program: Vec<Token> = Vec::new();
    for c in input.chars() {
        let token: Option<Token> = match c {
            '>' => Some(Token::IncDP),
            '<' => Some(Token::DecDP),
            '+' => Some(Token::IncMemory),
            '-' => Some(Token::DecMemory),
            '.' => Some(Token::Put),
            ',' => Some(Token::Read),
            '[' => Some(Token::While),
            ']' => Some(Token::EndWhile),
            _ => None // Ignore unknown characters in program
        };
        match token {
            Some(token) => parsed_program.push(token),
            None => ()
        }
    }
    parsed_program
}
        
fn open_and_parse(filename: &str) -> std::io::Result<()> {
    let content = read_file(filename)?;
    let parsed_content = parse(&content);
    simpleinterp(&parsed_content);
    Ok(())
}

fn main() {
    use std::time::SystemTime;
    let time = SystemTime::now();
    match open_and_parse("mandelbrot.bf") {
        Ok(_) => println!("Done!"),
        Err(err) => println!("Cannot read file mandelbrot.bf: {}", err)
    };
    println!("Duration: {:?}", time.elapsed().unwrap())
}

