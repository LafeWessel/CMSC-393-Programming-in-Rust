// create mini version of grep that finds and prints the lines in a file that match a keyword/phrase
// e.g. cargo run <phrase> <filename>
use std::env;
use std::fs;

fn main() {
    let args : Vec<String> = env::args().collect();
    println!("Args {:?}", args);
    let phrase = &args[1];
    let filename = &args[2];
    println!("Phrase: {}", phrase);
    println!("Filename: {}", filename);

    let contents = fs::read_to_string(filename).expect("Unable to read file");

    println!("File contents:\n{}", contents);
}





