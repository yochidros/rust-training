use std::fs::File;
use std::io::{Error, Read};

use std::fmt::Display;

struct User {
    name: String,
}

impl Display for User {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({})", self.name)
    }
}

fn main() {
    let vet = vec![1, 23, 3, 4, 5];
    let ten_item = vet.get(1);
    if let Some(&i) = ten_item {
        println!("{}", i);
    } else {
        panic!("not exsist");
    }
    println!(
        "{}",
        User {
            name: String::from("test")
        }
    );
    match read_file() {
        Ok(s) => {
            println!("{}", s);
        }
        Err(e) => {
            panic!(e);
        }
    }
}

fn read_file() -> Result<String, Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
