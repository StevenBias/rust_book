use std::io;                // Input io library
use rand::Rng;              // Input library to generate rand
use std::cmp::Ordering;     // For number comparison

struct Guess {
    value: u8,
}

impl Guess {
    fn new(value: u8) -> Guess {
        if value > 100 {
            panic!("The value is greater than 100!");
        }
        Guess {
            value
        }
    }
    // Getter
    fn value(&self) -> u8 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");

    // gen_range function is inclusive on the lower bond but exclusive on the upper bond
    let secret_number = rand::thread_rng().gen_range(0, 101);

    loop {
        println!("Please, input your guess: ");

        let mut guess = String::new();          // Mutable variable
        io::stdin().read_line(&mut guess)       // &mut guess instead of &guess to make it mutable
            .expect("Failed to read line");     // Catch error

        /*  Shadow previous string guess variable
         *
         *  trim    remove whitespaces at the bebining and the end of a string
         *  parse   parse string into number, catch an error in case the string does not contain number
         */
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => {
                println!("Your number is not bewtween 0 and 100!");
                continue;            // '_' catch all value
            },
        };
        let guess = Guess::new(guess);

        match guess.value().cmp(&secret_number) {
            Ordering::Less    => println!("It's higher"),
            Ordering::Greater => println!("It's lower"),
            Ordering::Equal   => {
                println!("You win!");
                break;
            }
        }
    }
    println!("The secret number is: {}", secret_number);
}
