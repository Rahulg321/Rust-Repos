pub fn for_element_loop(nums: &[i32]) {
    println!("reverse loop");
    for number in (1..4).rev() {
        println!("num {number}")
    }

    println!("simple loop");
    for number in (1..4) {
        println!("num {number}")
    }

    for element in nums {
        println!("element is {element}");
    }
}

pub fn while_loop() {
    let mut number = 2;

    while number != 10 {
        println!("num {number}");
        number += 1;
    }

    println!("list offf!!!");
}

pub fn simple_loop() {
    let mut counter = 0;

    let result = loop {
        println!("counter is {counter}");
        counter += 1;

        if counter == 10 {
            // breaking and returning a value
            break counter * 2;
        }
    };

    println!("result {result}");
}

pub fn loop_label() {
    // assigning a label to a loop

    let mut count = 0;

    'counting_down: loop {
        break 'counting_down;
    }

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        'remaining_down: loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break 'remaining_down;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
