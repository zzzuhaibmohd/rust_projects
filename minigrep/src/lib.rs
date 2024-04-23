use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        
        
        args.next();
        
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didnt get a query string")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didnt get a filename string")
        };

        let case_sensitive = env::var(" ").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line |line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if (line.to_lowercase().contains(&query)) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "fast";
        let contents = "Rust: \nsafe, fast\n Productive, Pick Three ";

        assert_eq!(vec!["safe, fast"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\nRust\n: asd \nsafe, fast\n Productive, Pick Three ";

        assert_eq!(vec!["Rust"], search_case_insensitive(query, contents));
    }
}
