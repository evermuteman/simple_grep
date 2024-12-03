
use std::error::Error;
use std::fs;


pub struct Config{
    pub file_name: String,
    pub file_path: String
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3{
            return Err("no enough args");
        }
        let file_name = args[1].clone();
        let file_path = args[2].clone();

        return Ok(Config { file_name:file_name, file_path:file_path });
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.file_path)?;
    println!("Got it!\nYour file contains:\n{content}\nSuch beautiful text!");
    Ok(())
}