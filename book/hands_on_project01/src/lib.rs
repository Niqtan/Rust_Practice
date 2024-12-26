use std::{fs, error::Error, env};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {   
    pub fn build(mut to_parse: impl Iterator <Item = String>) -> Result<Config, &'static str> {
        to_parse.next();

        let mut ignore_case = false;

        let file_path = match to_parse.next() {
            Some(arg) => arg,
            _ => return Err("Didn't get a file path"),
        };


        let query = match to_parse.next() {
            Some(arg) if !arg.starts_with("--") => arg,
            _ => return Err("No query string"),
        };
            
        if env::var("IGNORE_CASE").is_ok() {
            ignore_case = true;
        }
        else if env::var("IGNORE_CASE").is_err() {
            for arg in to_parse {
                if arg.contains("--ignore-case") {
                    ignore_case = true
                }
            }
        }

        Ok(Config {query, file_path, ignore_case})
}
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case{
        search_case_insensitive(&config.query, &contents)
    }
    else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    content
        .lines()
        .filter(|line| line.contains(&query))
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";


        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}