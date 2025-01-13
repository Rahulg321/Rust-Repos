struct Ipv4Addr {
    // --snip--
    address: (u8, u8, u8),
    owner: String,
}

struct Ipv6Addr {
    address: (u8, u8, u8, u8),
    owner: String,
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Option<T> {
    None,
    Some(T),
}

fn enum2_main() {
    println!("inside main");

    let ipv4 = Ipv4Addr {
        address: (192, 168, 0),
        owner: String::from("Alice"),
    };

    let ipv6 = Ipv6Addr {
        address: (0, 0, 0, 1),
        owner: String::from("Bob"),
    };

    let ip_v4 = IpAddr::V4(ipv4);
    let ip_v6 = IpAddr::V6(ipv6);
}
