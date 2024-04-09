use std::{process , fs ,env};
use std::error::Error;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."],search(query,contents))
    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "\
        Rust:
safe, fast, productive.
Pick three.
Trust me.
        ";
        assert_eq!(vec!["Rust:","Trust me."],
        search_case_insensitive(query,contents));
    }
}

pub fn run(config: Config) ->Result<(),Box<dyn Error>>{
    let content = fs::read_to_string(config.file_path)?;
    // println!("With text:\n{content}");
    let results = if config.ignore_case {
        search_case_insensitive(&config.query,&content)
    }else{
        search(&config.query,&content)
    };

    for line in results{
        println!("{line}")
    }
    Ok(())
}

pub struct Config{
    pub query: String,
    pub file_path: String,
    pub ignore_case : bool,
}

impl Config {
    pub fn build(mut args : impl Iterator<Item = String>)->Result<Config,& 'static str>{
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a query")};

        let file_path = match  args.next() {
            Some(arg) => arg,
            None => return Err("Did not get a file")};

        let ignore_case = env::var("IGNORE_CASE").map_or(false,|var| var.eq("1"));

        Ok(Config{query,file_path,ignore_case})
    }
}

pub fn parse_config(args : &[String])->Config{
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config{query,file_path, ignore_case: false }
}

fn search<'a>(query : &str,contents : &'a str)->Vec<&'a str>{
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query : &str,content : &'a str)->Vec<&'a str>{
    let query = query.to_lowercase();
    let mut result = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    return result;
}