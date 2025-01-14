fn iterate_string_chars(s: &String) {
    for c in s.chars() {
        println!("c -> {c}");
    }
}

fn get_first_word_before_space(s: &String) -> &str {
    for (i, c) in s.chars().enumerate() {
        if c == ' ' {
            return &s[..i]; // Return the slice before the space
        }
    }
    &s // Return the whole string if no space is found
}

fn c2_main() {
    for b in "Зд".bytes() {
        println!("{b}");
    }

    iterate_string_chars(&String::from("hello world"));

    let greeting = String::from("नमस्ते");
    // iterate_string_chars(&greeting);
    println!("length of greeting {}", greeting.len());

    let first_byte = greeting.as_bytes()[0]; // Accesses the first byte
    println!("First byte: {}", first_byte); // Output: 224 (not 'न')

    let second_byte = greeting.as_bytes()[1];
    println!("second byte: {}", second_byte); // Output: 164

    let hello = String::from("Здравствуйте");
    iterate_string_chars(&hello);
    let hello = String::from("नमस्ते");
    iterate_string_chars(&hello);

    let mut s1 = String::from("foo");
    let s2 = "some value";
    s1.push_str("bar");
    s1.push_str(s2);

    println!("s1 = {s1}");
    println!("s2 = {s2}");

    // s1 ownerships has been moved
    /*

    + Operator for String: In Rust, the + operator is defined for String in a specific way. It takes ownership of the left-hand side operand (s1 in this case) and borrows the right-hand side operand (s2).

    in this case for example we can no longer access s1 after use

    */
    let s3 = s1 + s2;
    println!("s3 = {s3}");

    println!("after adding");
    // println!("s1 = {s1}");
    println!("s2 = {s2}");

    let string1 = String::from("tic");
    let string2 = String::from("tac");
    let string3 = String::from("toe");

    /*

           if we want to combine multiple strings in more complex manner we use the format! macro
    The format! macro works like println!, but instead of printing the output to the screen, it returns a String with the contents
        */
    let result_string = format!("{string1}-*{string2}-*{string3}-*");

    println!("result_string -> {result_string}")
}
