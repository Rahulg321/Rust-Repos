// exporting entire modules
pub mod back_of_house;
pub mod front_of_house;
pub mod kitchen; // Declares the `kitchen` module
pub mod utils; // Declares the `utils` module

// Re-exporting specific modules or items or functions from a module using
pub use front_of_house::hosting; // Expose `hosting` directly
pub use front_of_house::serving::take_order; // Expose only `take_order`
