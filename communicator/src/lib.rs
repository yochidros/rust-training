mod network;

pub mod client;

pub mod outermost;

#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn say_hello() {
        assert_eq!("hello", client::say());
    }
}
