fn main() {
    println!("***Shadowing***");
    //  Shadowing
    let x = 5;
    let x = x + 1;
    println!("The value of x is: {}", x);
    let x = 5;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // Use shadowing to cast
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The number of spaces is: {}", spaces);

    /*  Compounds   */
    println!("\n***Shadowing***");
    //  Tuples
    println!("**Tuples**");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
}
