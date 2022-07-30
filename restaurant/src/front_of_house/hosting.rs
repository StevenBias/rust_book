pub use crate::back_of_house::Appetizer;

fn seat_at_table() {}
pub fn add_to_waitlist(order: Appetizer) {
    println!("Order {:?} added to wailtist", order);
}

