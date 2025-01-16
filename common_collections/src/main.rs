use std::collections::linked_list;
use std::collections::HashMap;
use std::collections::HashSet;

fn median_and_mode(numbers: &Vec<i32>) -> (f64, Vec<i32>) {
    if numbers.is_empty() {
        return (0.0, Vec::new()); // Handle empty input
    }

    let mut sorted_numbers = numbers.clone();
    sorted_numbers.sort();

    let mid = sorted_numbers.len() / 2;
    let median = if sorted_numbers.len() % 2 == 0 {
        (sorted_numbers[mid - 1] as f64 + sorted_numbers[mid] as f64) / 2.0
    } else {
        sorted_numbers[mid] as f64
    };

    let mut counts = HashMap::new();
    for &num in numbers {
        *counts.entry(num).or_insert(0) += 1;
    }

    let mut max_count = 0;
    let mut modes = Vec::new();

    for (&num, &count) in &counts {
        if count > max_count {
            max_count = count;
            modes.clear();
            modes.push(num);
        } else if count == max_count {
            modes.push(num);
        }
    }

    (median, modes)
}

fn update_or_insert() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}

fn main() {
    let vowels: HashSet<char> = "aeiouAEIOU".chars().collect();
    println!("Vowels are {vowels:?}");

    update_or_insert();

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    // check first if value exists or not only then insert
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");

    let team_name = String::from("Blue");
    /*
        .copied()
    Since your HashMap stores i32 values (which are simple numbers), .copied() creates an actual copy of the i32 if it's found. This avoids more complex borrowing situations.
    3. .unwrap_or(0)

    This is how you handle the possibility of None.
    unwrap_or(0) says:
    If you got Some(value), give me the value inside (10 in this case).
    If you got None (key not found), give me this default value of 0
         */
    let score1 = scores.get(&team_name).copied().unwrap_or(0);
    println!("score 1  {score1:?}");

    let score = scores.get_mut(&team_name);

    match score {
        Some(s) => {
            println!("score for team {} is {}", team_name, s);
            println!("adding 5 more to blue team score");
            *s += 5;
            println!("score after adding is {}", s);
        }
        None => println!("Could not find score for this team"),
    }

    if let Some(s) = scores.get_mut(&String::from("Yellow")) {
        *s = 25; // Change the score for "Blue" to 25
        println!("Updated score for Blue to 25");
    }

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("America");
    let field_value = String::from("Country");

    let mut map = HashMap::new();

    // map.insert(field_name, field_value); this transfer ownership of the those variables
    map.insert(field_name.clone(), field_value.clone());

    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    for (key, value) in &map {
        println!("{key}: {value}");
    }
    // println!("field name is {field_name}");
}
