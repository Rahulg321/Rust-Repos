pub fn data_types() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();

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
