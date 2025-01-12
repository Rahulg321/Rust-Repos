pub fn makes_copy(num: i32) {
    println!("some integer {num}");
}

pub fn take_string_ownership(s: String) {
    println!("took ownership {s}");
}

pub fn not_take_string_ownership(s: &str) {
    println!("did not take ownership {s}");
}

fn change(s: &mut String) {
    s.push_str("at the end");
}

fn calculate_length(s: &String) -> usize {
    // we cannot change a variable we have referenced or we dont have any ownership off
    // s.push_str("gupta");
    s.len()
}

// fn dangle() -> &String {
//     // the variable goes out of scope when the function ends, hence we get the error
//     let s = String::from("some value");
//     &s
// }

pub fn ownership_example1() {
    let x = 5;
    let y = x;

    // primitive types so they are just pushed on to the stack
    println!("{x}");
    println!("{y}");

    // run time allocation and not fixed size
    let s1 = String::from("julianne");
    let s2 = s1;

    // creates a deep copy, copying the entire contents of string from the heap
    let s3 = s2.clone();

    // here s1 will go out of bounds since memory was taken from heap and the ptr was moved
    // println!("{s1}");
    println!("s2 {s2}");
    println!("s3 {s3}");
}
