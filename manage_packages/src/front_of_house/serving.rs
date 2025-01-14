pub fn take_order(order_id: u32) {
    println!("Taking order {order_id}");
}

pub fn serve_order(order_id: u32) {
    println!("Serving order {order_id}");
}

pub fn take_payment(amount: f64) {
    println!("Payment of ${amount:.2} received");
}
