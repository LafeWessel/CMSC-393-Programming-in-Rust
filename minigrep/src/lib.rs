
use std::fs;
use std::env;
use std::error::Error;

pub struct Config{
    pub phrase : String,
    pub filename : String,
    pub case_sensitive : bool,
}

impl Config {
    pub fn new(args : &[String]) -> Result<Config, &str>{
        if args.len() < 3{
            return Err("Not enough arguments provided");
        }

        let phrase = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        return Ok(Config {phrase, filename, case_sensitive});
    }
}

pub fn run(config : Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive{
        search(&config.phrase,&contents)
    }else{
        search_case_insensitive(&config.phrase, &contents)
    };

    for line in results{
        println!("{}", line);
    }

    return Ok(());
}

pub fn search<'a> (query : &str, contents : &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.contains(&query){
            results.push(line);
        }
    }
    return results;
}

pub fn search_case_insensitive<'a>(query: &str, contents : &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    return results;
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result(){
        let q = "nobody";
        let c = "nobody wants\nto be an outcast";

        assert_eq!(vec!["nobody wants"], search(q,c));
    }

    #[test]
    fn case_insensitive(){
        let q = "NobOdY";
        let c = "nobody wants\nto be an outcast";

        assert_eq!(vec!["nobody wants"], search_case_insensitive(q,c));
    }

    #[test]
    fn case_sensitive(){
        let q = "Nobody";
        let c = "Nobody wants\nnobody else\nto be an outcast";

        assert_eq!(vec!["Nobody wants"], search(q,c));
    }
}