use std::io;        // Input io library
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Please, input your guess: ");

    let secret_number = rand::thread_rng().gen_range(0, 101);   // gen_range function is inclusive on the lower bond but exclusive on the upper bond

    let mut guess = String::new();           // Mutable variable
    io::stdin().read_line(&mut guess)       // &mut guess instead of &guess to make it mutable
        .expect("Failed to read line");     // Catch error

    println!("The secret number is: {}", secret_number);
}
