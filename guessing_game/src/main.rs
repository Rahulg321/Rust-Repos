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

fn route(ip_kind: IpAddrKind) {
    println!("version is {:?}", ip_kind);
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("self is {:#?}", self);
    }
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Received Quit message"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
    }
}

enum IpAddress {
    V4(String),
    V6(String),
}

fn main() {
    let homeIp = IpAddress::V4(String::from("127.0.0.1"));

    let home = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("ipaddress of home is is {:#?}", home);

    let m = Message::Write(String::from("hello"));
    m.call();
}
