fn find_user(user_id: u32) -> Option<String> {
    match user_id {
        1 => Some(String::from("User One")),
        10 => Some(String::from("User Ten")),
        _ => None,
    }
}

fn plus_one(v: Option<i32>) -> Option<i32> {
    match v {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)]
enum Dice {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

fn catch_all_pattern(d: Dice) {
    match d {
        Dice::Three => {
            println!("dice is three");
            add_fancy_hat();
        }
        Dice::Six => {
            println!("dice is six");
            remove_fancy_hat();
        }
        other => {
            move_player(other);
        }
    }
}

fn another_catch_all_pattern(d: Dice) {
    match d {
        Dice::Three => {
            println!("dice is three");
            add_fancy_hat();
        }
        Dice::Six => {
            println!("dice is six");
            remove_fancy_hat();
        }
        // when we dont the value in the catch all
        _ => {
            reroll();
        }
    }
}

fn nothing_catch_all_pattern(d: Dice) {
    match d {
        Dice::Three => {
            println!("dice is three");
            add_fancy_hat();
        }
        Dice::Six => {
            println!("dice is six");
            remove_fancy_hat();
        }
        // when we dont the value in the catch all and want to do nothing
        _ => (),
    }
}

fn reroll() {}
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: Dice) {
    println!("move player to {num_spaces:?}");
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
struct Question {
    id: u32,
    exam_name: String,
}

#[derive(Debug)]
enum QuestionType {
    MCQ(Question),
    FillInTheBlank(Question),
}

fn print_question(q: QuestionType) {
    match q {
        QuestionType::MCQ(Question) => {
            println!("Your question is an {Question:?}");
        }
        QuestionType::FillInTheBlank(Question) => {
            println!("Your question is an {Question:?}");
        }
    }
}

fn value_in_cents(c: Coin) -> u8 {
    match c {
        Coin::Dime => 10,
        Coin::Nickel => 5,
        Coin::Penny => 1,
        Coin::Quarter(UsState) => {
            println!("us state is {UsState:?}");
            25
        }
    }
}

fn options_main() {
    let d1 = Dice::Six;
    let d2 = Dice::Three;
    let d3 = Dice::One;

    catch_all_pattern(d1);
    catch_all_pattern(d2);
    catch_all_pattern(d3);

    let five = Some(5);
    let six = plus_one(five);

    let none = plus_one(None);

    println!("six is {six:?} and none is {none:?}");

    let mcq = Question {
        id: 1,
        exam_name: "Math Exam".to_string(),
    };

    let fill_in_blank = Question {
        id: 2,
        exam_name: "English Exam".to_string(),
    };

    print_question(QuestionType::MCQ(mcq));
    print_question(QuestionType::FillInTheBlank(fill_in_blank));

    let c1 = Coin::Quarter(UsState::Alaska);

    let value = value_in_cents(c1);
    println!("value is {value}");

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
