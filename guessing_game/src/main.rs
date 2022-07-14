use std::io;        // Input io library

fn main() {
    println!("Guess the number!");
    println!("Please, input your guess: ");

    let mut guess = String::new();           // Mutable variable
    io::stdin().read_line(&mut guess)       // &mut guess instead of &guess to make it mutable
        .expect("Failed to read line");     // Catch error

    println!("You guessed: {}", guess);
}
