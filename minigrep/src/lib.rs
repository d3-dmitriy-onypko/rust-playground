use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    query: String,
    filename: String,
    ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        match args.len() {
            3 => Ok(Config {
                query: args[1].clone(),
                filename: args[2].clone(),
                ignore_case,
            }),
            i if i > 3 => Err("too many arguments"),
            _ => Err("Usage: {{query}} {{filename}}"),
        }
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut vec = vec![];
    for line in contents.lines().filter(|line| line.len() >= query.len()) {
        if line.contains(query) {
            vec.push(line);
        }
    }

    vec
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
