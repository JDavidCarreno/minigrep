use std::{env, process};

use minigrep::{run, Config};

fn main() {
    let args : Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In the file: {}", config.pathname);

    if let Err(e) = run(config) {
        eprintln!("App error: {}", e);
        process::exit(1);
    }
}
