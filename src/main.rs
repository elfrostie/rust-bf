use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const MEMORY_SIZE: usize = 30000;

fn calculate_jumptable(program: &str) -> Vec<usize> {
    let instructions: Vec<char> = program.chars().collect();
    let mut jumptable: Vec<usize> = vec![0; instructions.len()];
    let mut pc: usize = 0;

    while pc < instructions.len() {
        let instruction: char = instructions[pc];
        if instruction == '[' {
            let mut bracket_nesting = 1;
            let mut seek = pc;

            while bracket_nesting > 0 && seek < instructions.len() - 1 {
                seek += 1;
                if instructions[seek] == ']' {
                    bracket_nesting -= 1;
                } else if instructions[seek] == '[' {
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

fn simpleinterp(program: &str) {
    let mut memory: Vec<u8> = vec![0; MEMORY_SIZE];
    let jumptable = calculate_jumptable(&program);
    let mut pc: usize = 0;
    let mut dataptr: usize = 0;
    let instructions: Vec<char> = program.chars().collect();
    while pc < instructions.len() {
        let instruction: char = instructions[pc] as char;
        match instruction {
            '>' => dataptr = dataptr.wrapping_add(1),
            '<' => dataptr = dataptr.wrapping_sub(1),
            '+' => memory[dataptr] = memory[dataptr].wrapping_add(1),
            '-' => memory[dataptr] = memory[dataptr].wrapping_sub(1),
            ',' => panic!(", not implemented"),
            '[' => {
                if memory[dataptr] == 0 {
                    pc = jumptable[pc];
                }
            },
                        
            ']' => {
                if memory[dataptr] != 0 {
                    pc = jumptable[pc];
                }
            },
            unknown_symbol => panic!("Unknown symbol {}", unknown_symbol)
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

fn parse(input: &str) -> String {
    let mut parsed_str = String::with_capacity(input.len());
    for c in input.chars() {
        match c {
            '>' | '<' | '+' | '-' => parsed_str.push(c),
            '.' | ',' | '[' | ']' => parsed_str.push(c),
            _ => ()
        }
    }
    parsed_str
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

