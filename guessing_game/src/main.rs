fn main() {
    let config_max: Option<u8> = None;
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    } else {
        println!("value is not present");
    }

    let dice_value = 3;

    if let value = dice_value {
        println!("dice is {value}");
    }
}
