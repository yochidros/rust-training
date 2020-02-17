use std::io;

fn main() {
    println!("Hello, Find Word Program!");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("unknown input value");

    println!("input text: {}", input);
    let mut vec: Vec<String> = Vec::new();
    let end: usize = input.len();
    let mut start: usize = 0;
    loop {
        let (word, s) = first_word(&input[start..end]);
        start += s + 1;
        vec.push(word.to_string());
        if end <= start {
            break;
        }
    }
    println!("find words following..");
    for item in vec.iter() {
        println!("{}", item);
    }
}

fn first_word(s: &str) -> (&str, usize) {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return (&s[..i], i);
        }
    }
    (&s[..], s.len())
}
