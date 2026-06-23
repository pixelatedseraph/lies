use::std::fs::File;
use::std::io::{BufRead,BufReader};
use std::vec;

use crate::Instruction;

pub enum TokenKind<'a>{
    load(&'a str,i64),
    conswrite(&'a str),
    cmp(&'a str,&'a str),
    jne(i32),
    inc(&'a str),
    halt,
    eof,
    err,
} 

pub struct Lexer<'a>{
    source_code   : &'a str,
    current_index : usize,
} 

/* impl <'a> Lexer<'a> {
    pub fn new() -> Self{
        Self{
            source_code : 
        }
    }
} */


pub fn to_instructions_list(path : &str) -> std::io::Result<Vec<String>>{
    let file = File::open(path)?;

    let mut instructions_vec:Vec<String> = Vec::new();

    let reader  = BufReader::new(file);
    for line in reader.lines(){
        let line = line?;
        instructions_vec.push(line);    
    }

    Ok(instructions_vec)
}


fn parse_reg(s: &str) -> usize{
    s.strip_prefix('R')
        .unwrap()
        .parse()
        .unwrap()
}


pub fn parse_line(line : &str) -> Instruction{
    let parts: Vec<_> = line.split_ascii_whitespace().collect();

    match parts.as_slice() {
        ["load",reg,value] => {
            Instruction::Load(parse_reg(*reg), value.parse().unwrap())
        }
        ["inc",reg] => {
            Instruction::Inc(parse_reg(*reg))
        }
        ["conswrite",reg] => {
            Instruction::Print(parse_reg(*reg))
        }
        ["cmp",r1,r2] => {
            Instruction::Cmp(parse_reg(*r1),parse_reg(*r2))
        }
        ["jne",target] => {
            Instruction::Jne(target.parse().unwrap())
        }
        ["halt"] => {
            Instruction::Halt
        }
        _ => panic!("Unsupported Instruction")
    }
}


pub fn dump_instructions_from_file(path :&str) -> Vec<Instruction>{
    let mut result_vec:Vec<Instruction> = Vec::new();

    let instructions_vec = to_instructions_list(path).expect("[Error] to_instructions_list failed perfoming IO");
    for string in instructions_vec.iter(){
        result_vec.push(parse_line(string));
    }
    result_vec
}

