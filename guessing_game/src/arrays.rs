fn array_indexing() {
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    println!("months are {:?}", months);
    println!("first is {:?}", first);
    println!("second is {:?}", second);

    let mut userIndex = String::new();
    println!("enter the index you want to access");
    io::stdin()
        .read_line(&mut userIndex)
        .expect("could not read line for cmd");

    let userIndex: usize = userIndex
        .trim()
        .parse()
        .expect("please enter a valid number");

    let element = a[userIndex];
    println!("The value of the element at index {userIndex} is: {element}");
}
