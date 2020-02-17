pub mod args {

    pub struct Config<'a> {
        pub query: String,
        pub filenames: &'a [String],
    }

    impl Config<'_> {
        pub fn new(args: &[String]) -> Result<Config, &'static str> {
            if args.len() < 3 {
                return Err("ERROR: args needs (search query) filename...");
            }
            let query = args[1].to_string();
            let filenames: &[String] = &args[2..];
            Ok(Config { query, filenames })
        }
    }
}

pub mod fileio {
    use std::fs::File;
    use std::io::{prelude::*, Error};
    fn read_file(filename: &str) -> Result<File, Error> {
        let f = File::open(filename)?;
        Ok(f)
    }

    pub fn read_text(filename: &str) -> Result<String, Error> {
        let mut file = read_file(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}

pub mod search {

    pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut results = Vec::new();
        let query = query.to_lowercase();
        for line in contents.lines() {
            if line.to_lowercase().contains(&query) {
                results.push(line);
            }
        }
        results
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_results() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.query
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive.query"],
            search::search(query, contents)
        );
    }
}
