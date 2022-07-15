fn main() {
    let number= 3;

    // Do not use parenthesis for condition...
    // The condition MUST be a bool
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    let condition = false;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);
}
