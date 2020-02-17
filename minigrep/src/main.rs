use std::env;
use std::{fmt::Display, process};

use minigrep::args;
use minigrep::fileio;
use minigrep::search;

use std::fmt;
struct User {
    name: String,
}
impl Iterator for User {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.name.to_string())
    }
}
impl Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name.as_str())
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = args::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    let users = vec![
        User {
            name: String::from("a"),
        },
        User {
            name: String::from("b"),
        },
    ];

    for item in users.iter() {
        println!("{}", item);
    }

    println!("Searching fo {}", config.query);

    for filename in config.filenames.as_ref().iter() {
        println!("\n=> In File: {}\n", &filename);

        let text = match fileio::read_text(filename) {
            Ok(t) => t,
            Err(error) => {
                eprintln!("{} is not contains text: {}", filename, error);
                continue;
            }
        };

        let result = search::search(config.query.as_str(), text.as_str());
        if result.len() > 0 {
            println!("Find query {}!!!\n", filename);
            for item in result.iter() {
                println!("\t{}", item);
            }
        } else {
            println!("\nsorry, not find...\n");
        }
    }
    println!("\nEnd find query!!!!!! \n");
    process::exit(0);
}
