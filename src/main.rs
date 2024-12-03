
use std::env;
use std::process;

use simple_grep::run;
use simple_grep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Let me guess, you are looking for {}",config.file_name);
    println!("and I can find it in {}",config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

}


#[cfg(test)]
mod test{
    use super::*;
    
}
