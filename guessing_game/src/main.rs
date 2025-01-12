use core::slice;
use rand::Rng;
use std::cmp::Ordering;
use std::f64::consts::PI;
use std::io;

mod loops;
mod ownership;

const _THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

trait MathematicalObject {
    fn print(&self);
    fn calculate_perimeter(&self) -> f64;
    fn calculate_area(&self) -> f64;
}

struct Rectangle {
    length: f64,
    width: f64,
}

struct Square {
    side: f64,
}

impl MathematicalObject for Rectangle {
    fn calculate_perimeter(&self) -> f64 {
        2.0 * (self.length + self.width)
    }

    fn calculate_area(&self) -> f64 {
        self.length * self.width
    }

    fn print(&self) {
        println!("length is {}", self.length);
        println!("width is {}", self.width);
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

impl MathematicalObject for Square {
    fn calculate_perimeter(&self) -> f64 {
        4.0 * (self.side)
    }

    fn calculate_area(&self) -> f64 {
        3.0 * PI * self.side * self.side
    }

    fn print(&self) {
        println!("side of sqaure is {}", self.side);
    }
}

impl User {
    fn new() -> Self {
        User {
            active: false,
            username: String::new(),
            email: String::new(),
            sign_in_count: 1,
        }
    }

    fn with_username(mut self, username: String) -> Self {
        self.username = username;
        self
    }

    fn with_email(mut self, email: String) -> Self {
        self.email = email;
        self
    }

    fn with_active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

// unit like structs
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let mut user1 = User {
        active: false,
        username: String::from("surbhi"),
        email: String::from("surbhi@gmail.com"),
        sign_in_count: 12,
    };

    let user2 = User::new()
        .with_active(true)
        .with_email(String::from("rg5353070@gmail.com"))
        .with_username(String::from("_rahul321_"));

    println!("user2 is {:#?}", user2);

    let user3 = User {
        email: String::from("ninaDobrev@example.com"),
        ..user2
    };

    // we can now longer user user2 since we moved some heap string values over to user3
    // println!("user2 is {:?}", user2);

    println!("user3 is {:?}", user3);

    println!("{:?}", user1.email);
    user1.email = String::from("testing@gmail.com");
    println!("{:?}", user1.email);

    let rect1 = Rectangle {
        length: 12.4,
        width: 65.7,
    };

    let rect2 = Rectangle {
        length: 7.4,
        width: 31.7,
    };

    rect1.can_hold(&rect2);

    let sq1 = Square { side: 12.4 };

    let rect1_area = rect1.calculate_area();
    let rect1_perimiter = rect1.calculate_perimeter();

    rect1.print();

    let sq1_area = sq1.calculate_area();
    let sq1_perimeter = sq1.calculate_perimeter();

    sq1.print();

    println!(
        "rect1 area {:.2} and perimeter {:.2}",
        rect1_area, rect1_perimiter
    );

    println!(
        "sq1 area {:.2} and perimeter {:.2}",
        sq1_area, sq1_perimeter
    );
}
