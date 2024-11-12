use std::env;
use std::fs;
use std::process::exit;
use std::error::Error;


fn run_file(path:&str)->Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(path).expect("couldnt read file")?;

}

fn run_prompt(){

}


fn main(){
    let args:Vec<String> = env::args().collect();

    if args.len() > 2 {
        println("Usage:jlox [script");
    }else if args.len() == 2 {
        run_file(&args[1]);
    }else{
        run_prompt();
    }
}