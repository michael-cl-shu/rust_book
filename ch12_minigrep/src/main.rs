use std::env;
use std::process;

use ch12_minigrep::Config;
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In the file {}", config.filename);

    if let Err(e) = ch12_minigrep::run(config) {
        println!("Application Error: {}", e);
        process::exit(1);
    }
}
