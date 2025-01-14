use back_of_house::breakfast;
use manage_packages::front_of_house::{hosting, Customer, Order, SeatingPreference};
mod app;
mod back_of_house;
mod kitchen;
use app::cli::parse_args;
use manage_packages::take_order;
use rand::Rng;
use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // This is a sample implementation
    Ok(()) // Returns success without any value

    // Alternatively, to simulate an error:
    // Err(std::fmt::Error)
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    map.insert(2, 4);
    map.insert(3, 6);

    println!("current hashmap is {map:?}");

    app::setup::initialize();
    app::cli::parse_args();

    take_order(12);

    back_of_house::breakfast::eat_at_restaurant();

    let starting_meal = breakfast::Breakfast::summer("red berries");
    // Public struct with public fields
    let customer = Customer {
        name: "Alice".to_string(),
        table_number: 5,
    };
    println!(
        "Customer: {}, Table: {}",
        customer.name, customer.table_number
    );

    // Public struct with private fields
    let order = Order::new(1, vec!["Pasta".to_string(), "Salad".to_string()]);
    println!("{}", order.details());

    // Public enum
    let preference = SeatingPreference::Indoor;
    match preference {
        SeatingPreference::Indoor => println!("Customer prefers indoor seating."),
        SeatingPreference::Outdoor => println!("Customer prefers outdoor seating."),
    }
}
