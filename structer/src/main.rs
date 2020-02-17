#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn say_name(&self) -> &str {
        return self.username.as_str();
    }

    fn create(username: String, email: String) -> User {
        return User {
            username,
            email,
            active: false,
            sign_in_count: 0,
        };
    }
}

fn main() {
    let user1 = User {
        username: String::from("yochidros"),
        email: String::from("mm9.movement.trb@gmail.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("user1: {:?}", user1);
    println!("user1 say_name: {}", user1.say_name());
    let user2 = User { ..user1 };
    println!("user2: {:?}", user2);
    let user3 = User::create(
        String::from("yochidrop"),
        String::from("miyazawa@tribe0901.com"),
    );
    println!("user3: {:?}", user3);
}
