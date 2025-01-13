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
        match self {
            Message::Quit => {
                println!("The message is Quit");
            }

            Message::Move { x, y } => {
                println!("Message is Move with x = {}, y = {}", x, y);
            }
            Message::Write(text) => {
                println!("The message is Write with text: {}", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!(
                    "The message is ChangeColor with RGB values: r = {}, g = {}, b = {}",
                    r, g, b
                );
            }
        }
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

    let quit_message = Message::Quit;
    let move_message = Message::Move { x: 10, y: 20 };
    let write_message = Message::Write(String::from("Hello, world!"));
    let color_message = Message::ChangeColor(255, 0, 0);

    quit_message.call();
    move_message.call();
    write_message.call();
    color_message.call();
}
