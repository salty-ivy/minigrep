use std::env;
use std::fs;

struct Config {
    query: String,
    file_path: String,
}


impl Config {
    fn new(args: &Vec<String>)-> Config{
        if args.len() < 3 {
            panic!(" Not enough arguments");
        }
        let query: String = args[1].clone();
        let file_path: String = args[2].clone();
    
        Config{ query, file_path }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();  // args() call returns an iterator and .collect() will convert it into collection
    
    let config: Config = Config::new(&args);
    
    println!("Serching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents: String = fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("with text:\n\n{contents}");
}
