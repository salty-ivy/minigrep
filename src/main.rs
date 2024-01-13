use std::env;
use std::process;
use minigrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();  // args() call returns an iterator and .collect() will convert it into collection
    
    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}"); // stderr 
        process::exit(1);
    });


    if let Err(e) = minigrep::run(config){
        eprintln!("Application error: {e}"); // stderr
        process::exit(1);
    }
}

