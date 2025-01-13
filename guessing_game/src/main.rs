fn find_user(user_id: u32) -> Option<String> {
    match user_id {
        1 => Some(String::from("User One")),
        10 => Some(String::from("User Ten")),
        _ => None,
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("luckuy penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("us state is {state:?}");
            25
        }
    }
}

fn main() {
    let c1 = Coin::Quarter(UsState::Alaska);

    value_in_cents(c1);

    let user_id_to_find = 11; // Example: User not found
                              //let user_id_to_find = 1; // Example: User found

    let found_user = find_user(user_id_to_find);

    // 1. Using match (most explicit and often preferred):
    match found_user {
        Some(name) => println!("Found user (match): {}", name),
        None => println!("User with ID {} not found.", user_id_to_find),
    }

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    let x = 5;
    // let y = Some(5);
    let y: Option<i32> = None;

    let sum = match y {
        Some(val) => val + x,
        None => x,
    };

    println!("the sum is {sum}");
}
