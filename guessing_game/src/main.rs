use rand::Rng;
use std::cmp::Ordering;
use std::io;

const _THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
fn main() {
    another_function();
}

fn another_function() {
    println!("calling another function")
}

fn array_indexing() {
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    println!("months are {:?}", months);
    println!("first is {:?}", first);
    println!("second is {:?}", second);

    let mut userIndex = String::new();
    println!("enter the index you want to access");
    io::stdin()
        .read_line(&mut userIndex)
        .expect("could not read line for cmd");

    let userIndex: usize = userIndex
        .trim()
        .parse()
        .expect("please enter a valid number");

    let element = a[userIndex];
    println!("The value of the element at index {userIndex} is: {element}");
}

fn data_types() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();

    println!("THREE THREE_HOURS_IN_SECONDS is {THREE_HOURS_IN_SECONDS}");
    println!("spaces is {spaces}");

    let guess: u32 = match "42somevalue ".trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("an error occured");
            1 // providing a default value in case of error
        }
    };

    let y = 3.21;
    let z = -3.21;

    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    let c = 'Z';
    let single: char = 'Z';
    let _heart_eyed_cat = '😻';

    let tup = (1, 2, 3);
    let tup1 = (1, 2.1, 3.3, false, "hello");
    let greetings = ("hello", "HI", "BYE");

    let first = greetings.0;
    let second = greetings.1;
    let third = greetings.2;

    println!("greeting second {second}");
}

fn guessing_game() {
    println!("Welcome to guessing game");

    // generates a random number between a specified range
    // inclusive on lower and upper bounds
    let secret_number = rand::thread_rng().gen_range(1..=50);

    loop {
        println!("please input your guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // converting the string guess to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, //skip this iteration and move on the next
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!!"),
            Ordering::Equal => {
                println!("guessed correctly");
                println!("game over");
                break;
            }
            Ordering::Greater => println!("Too big!!!"),
        }
    }
}
