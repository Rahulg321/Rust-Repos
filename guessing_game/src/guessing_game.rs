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
