use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::process::exit;

fn run_file(path: &str) -> Result<(), String> {
    match fs::read_to_string(path) {
        Err(msg) => return Err(msg.to_string()),
        Ok(contents) => return run(&contents),
    }
}

fn run(contents: &str) -> Result<(), String> {
    return Err("Not Implemented");
}

fn run_prompt() -> Result<(),Error>{
    print!(">");
    let mut buffer = String::new();
    let mut handle = stdin.lock();
    let stdin = io::stdin();
    match handle.read_line(&mut buffer){
        Ok(_)=>(),
        Err(_)=>return Err("Couldn't read line".to_string());
    }
    println!("You wrote :{}",buffer);
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
                println!()
            }
        }
    }
}
