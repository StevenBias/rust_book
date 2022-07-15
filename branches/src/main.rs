fn main() {
    let number= 3;

    // Do not use parenthesis for condition...
    // The condition MUST be a bool
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    /***    If condition in let statement   ***/
    //  The arms must return same type value
    let condition = false;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);

    /***    Repetition with loops   ***/
    //  Returning values from loops
    let mut counter = 0;

    let mut result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {}", result);

    while result > 0 {
        result -= 4;
        println!("{}", result);
    }
    println!("LIFTOFF");

    let a = [1, 2, 3, 4, 5];
    for element in a.iter() {
        println!("The value is: {}", element);
    }

    //  Use range for for loop
    //  The higher border is excluded
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
