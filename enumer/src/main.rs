#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let v4 = IpAddrKind::V4;
    println!("{:?}", v4);
    let ip_addr = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("192.168.11.1"),
    };
    println!("{:?}", ip_addr);
    println!("{}", value_in_cents(&Coin::Penny));
    let res: Result<u32, std::io::Error> = Err(std::io::Error::from(std::io::ErrorKind::Other));

    if let Err(e) = res {
        println!("{}", e);
    }
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 2,
        Coin::Dime => 3,
        Coin::Quarter => 4,
    }
}
