fn control_flow() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
    if y < 4 {
        println!("y is less than 4");
    } else {
        println!("y is greater than 4");
    }

    let number = 3;

    if number != 0 {
        println!("number is not 0");
    }
}

fn five() -> i32 {
    5
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn print_array(arr: &[i32]) {
    for &item in arr {
        println!("{}", item);
    }
}

fn another_function(x: i32, msg: &str) {
    println!("calling another function {x}");
    println!("string arguement {msg}")
}
