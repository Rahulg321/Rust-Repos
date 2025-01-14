fn read_first_vector(v: &Vec<i32>) -> Option<&i32> {
    v.get(0)
}

fn read_vector_using_index(v: &Vec<i32>, i: usize) -> Option<&i32> {
    v.get(i)
}

fn read_all_vector(v: &Vec<i32>) {
    for i in v {
        println!("{i}");
    }
}

fn read_all_vector_strings(v: &Vec<&str>) {
    for s in v {
        println!("string is {s}");
    }
}

enum SpreadsheetCell {
    Empty, // Represents an empty cell
    Int(i32),
    Float(f64),
    Text(String),
    Boolean(bool),   // Represents a boolean value
    Date(String), // Represents a date (you might use a dedicated Date type in a real application)
    Formula(String), // Represents a formula
    Error(String), // Represents an error in a cell (e.g., #DIV/0!)
}

fn second_main() {
    let data = "initial contents";
    let string_data = data.to_string();
    let s = String::from(data);

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let row1 = vec![
        SpreadsheetCell::Text(String::from("Name")),
        SpreadsheetCell::Text(String::from("Age")),
        SpreadsheetCell::Text(String::from("Score")),
    ];

    let row2 = vec![
        SpreadsheetCell::Text(String::from("Alice")),
        SpreadsheetCell::Int(30),
        SpreadsheetCell::Float(95.5),
    ];

    let mut v: Vec<i32> = Vec::new();

    v.push(12);
    v.push(10);
    v.push(9);
    v.push(8);

    println!("{v:?}");

    let index_to_access: usize = 12;

    if let Some(r) = read_vector_using_index(&v, index_to_access) {
        println!("found something at index {index_to_access}");
    } else {
        println!("did not found at index {index_to_access}");
    }

    if let Some(result) = read_first_vector(&v) {
        println!("Result is {result:?}");
    } else {
        println!("No value found in vector.");
    }

    let vector_value = v.get(0);

    if let Some(val) = vector_value {
        println!("Vector value exists: {val:?}");
    } else {
        println!("Vector value does not exist.");
    }

    let mut v1 = vec![1, 2, 3];

    for i in &mut v1 {
        println!("mutable reference is {}", *i);
        *i *= 3;
    }

    let cities = vec![
        "Paris",
        "Amsterdam",
        "New York",
        "Copenhagen",
        "Tokyo",
        "Miami",
    ];

    read_all_vector(&v1);
    read_all_vector_strings(&cities);
}
