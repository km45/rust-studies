use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

// use "trait object" `Box<Error>>` explained in chapter 17
pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_config_fails_for_less_than_three_arguments() {
        {
            let args = vec![String::from("a")];
            let result = Config::new(&args);
            assert!(result.is_err());
        }
        {
            let args = vec![String::from("a"), String::from("b")];
            let result = Config::new(&args);
            assert!(result.is_err());
        }
    }

    #[test]
    fn create_config_succeeds_for_three_arguments() {
        let args = vec![String::from("a"), String::from("b"), String::from("c")];
        let result = Config::new(&args);
        assert!(result.is_ok());
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
