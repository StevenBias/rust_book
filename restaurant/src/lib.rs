// Tells Rust to load the contents of the module from another file with the
// same name as the module
mod front_of_house;
mod back_of_house;

// Absolute path
pub use crate::front_of_house::hosting;
// Relative path with self
// use self::front_of_house::hosting;
// Unidiomatic use Path
// use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

