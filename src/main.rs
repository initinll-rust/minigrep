use std::{env, fs, process, error::Error};

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problems parsing arguments : {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs:: read_to_string(config.file_path)?;
    println!("With text: \n {contents}");
    Ok(())
}

struct Config {
    query: String,
    file_path: String
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &str> {

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        
        let config = Config { 
            query, 
            file_path 
        };

        Ok(config)
    }
}