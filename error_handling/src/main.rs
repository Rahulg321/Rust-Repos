use dotenv::dotenv;
use std::env;
use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    let result = read_username_from_file().expect("An error occured while calling func");
    println!("result {result}");
}

fn read_username_from_file() -> Result<String, Error> {
    let file_result = File::open("hello.txt");

    let mut username_file = match file_result {
        Ok(fl) => fl,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    // if we are successfully able to convert to string and read from file
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn unwrap_expect() {
    // unwrap panics if error otherwise returns the result no need to use match statement
    let greeting_file = File::open("hello.txt").unwrap();
    println!("greeting_file {greeting_file:?}");

    let test_file = File::open("test.txt").expect("we are expecting a test file");
    println!("test_file {test_file:?}");
}

fn read_or_create_using_closure() {
    let opencreatefile = |filename: &str| -> Result<File, std::io::Error> {
        match File::open(filename) {
            Ok(fl) => Ok(fl),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create(filename) {
                    Ok(fl) => Ok(fl),
                    Err(e) => Err(e),
                },
                other_error => Err(error),
            },
        }
    };

    // Use the closure to open "hello.txt"
    let greeting_file = opencreatefile("hello.txt").unwrap(); // unwrap for simplicity

    println!("greeting_file is {greeting_file:?}");
}

fn read_or_create_file() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => {
            println!("File already exists, successfully read it");
            file
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Could not find file as well as create it!!!"),
            },
            // all other errors
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };
}

fn loading_env_variables() {
    dotenv().ok(); // Load environment variables from .env

    // unwrap provides a default value if the result fails
    let value = env::var("MY_ENV_VAR").unwrap_or("default_value".to_string());

    println!("Value: {}", value);

    match env::var("MY_ENV_VAR") {
        Ok(val) => println!("my MY_ENV_VAR is {val}"),
        Err(e) => println!("Could not read env variable {e}"),
    }
}
