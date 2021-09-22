// create mini version of grep that finds and prints the lines in a file that match a keyword/phrase
// e.g. cargo run <phrase> <filename>
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args : Vec<String> = env::args().collect();
    let configuration : Config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Error while parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(configuration){
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

