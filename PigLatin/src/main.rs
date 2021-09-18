//Convert strings to pig latin. The first consonant of each word is moved to the end of the word
// and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay”
// added to the end instead (“apple” becomes “apple-hay”).
// Keep in mind the details about UTF-8 encoding!
use std::io;
const vowels : [char;5] = ['a','e','i','o','u'];

fn main() {
    while true{
        let mut line = String::new();
        println!("Enter a series of words to change to PigLatin, or enter exit to quit");
        io::stdin().read_line(&mut line).expect("Unable to read line");
        line.pop();

        if line == "exit"{
            break;
        }
        let mut words : Vec<&str> = line.split(" ").collect();
        println!("words entered:{:?}",words);
        let mut latin_words : Vec<String> = Vec::new();
        for w in words.iter() {
            latin_words.push(latinise_word(&w));
        }
        println!("Latinised words:\n{:?}",latin_words);
        latin_words.clear();
        words.clear();
    }
    println!("Exited");
}

fn latinise_word(word : &str) -> String{
    // check if first character is a vowel
    word.to_lowercase();
    let mut result : String = String::new();
    if vowels.contains(&word.chars().nth(0).unwrap()){
        result = word.to_owned() + "-hay";
    }else{
        result = word[1..].to_owned() + "-" + &word.chars().nth(0).unwrap().to_string() + "ay";
    }

    return result;
}