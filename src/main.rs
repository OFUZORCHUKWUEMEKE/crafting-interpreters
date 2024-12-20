mod scanner;
mod traits;
mod treatt;
mod trits;
use crate::scanner::*;

use std::env;
use std::error::Error;
use std::fs;
use std::io::{self,BufRead,Write};
use std::process::exit;

fn run_file(path: &str) -> Result<(), String> {
    match fs::read_to_string(path) {
        Err(msg) => return Err(msg.to_string()),
        Ok(contents) => return run(&contents),
    }
}

fn run(contents: &str) -> Result<(), String> {
    let mut scanner = Scanner::new(contents);
    let tokens = scanner.scan_tokens()?;

    for token in tokens{
        println!("{:?}",token);
    }
    return Err("Not Implemented").to_string();
} 

fn run_prompt() -> Result<(),String>{ 
    loop  {
        print!(">");
    
        match io::stdout().flush(){
            Ok(_)=>(),
            Err(_)=>return Err("Could not flush stdout".to_string()),
        };
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        match handle.read_line(&mut buffer){

            Ok(n)=>{
                if n <=1 {
                    return Ok(());
                }
            },
            Err(_)=> return Err("Couldn't read line".to_string()),
            
        }
        println!("You wrote : {}", buffer);
        match run(&buffer){
            Ok(_)=>(),
            Err(msg)=>println!("{}",msg),
        }
    }  
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: Jlox [script]");
        exit(64);
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("Errors");
                exit(1);
            }
        }
    }else{
        match run_prompt(){
            Ok(_)=> exit(0),
            Err(msg)=>{
                println!("Errors");
                exit(1);
            }
        }
    }
}
