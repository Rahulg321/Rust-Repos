pub mod cook;
pub mod hosting; // Expose the `hosting` submodule
pub mod prep;
pub mod serving; // Expose the `serving` submodule

// A public struct with public fields
pub struct Customer {
    pub name: String,
    pub table_number: u32,
}

// A public struct with private fields
pub struct Order {
    pub id: u32,        // Public field
    items: Vec<String>, // Private field
}

// Implementation block for the `Order` struct
impl Order {
    // Public method to create a new order
    pub fn new(id: u32, items: Vec<String>) -> Self {
        Order { id, items }
    }

    // Public method to get order details
    pub fn details(&self) -> String {
        format!("Order ID: {}, Items: {:?}", self.id, self.items)
    }
}

// A private struct (not accessible outside this module)
struct Receipt {
    total_amount: f64,
}

// A public enum
pub enum SeatingPreference {
    Indoor,
    Outdoor,
}

// A private enum
enum WaitlistStatus {
    Waiting,
    Seated,
}

// Implementation block for `WaitlistStatus`
impl WaitlistStatus {
    fn status_message(&self) -> &str {
        match self {
            WaitlistStatus::Waiting => "Customer is waiting.",
            WaitlistStatus::Seated => "Customer has been seated.",
        }
    }
}
