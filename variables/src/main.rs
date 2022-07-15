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
    println!("The value of the first field of tup is: {}",  tup.0);
    println!("The value of the second field of tup is: {}", tup.1);
    println!("The value of the third field of tup is: {}",  tup.2);

    let (x, y, z) = tup;                                // Destructure tup in 3 variables
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    //  Array
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [6, 7, 8, 9, 10];
    let c = [3; 5];          // [3, 3, 3, 3, 3]
    let months = ["January", "February", "March", "April", "May", "June", "July", "August",
                    "September", "October", "November", "December"];
    println!("Array a: {:?}", a);
    println!("Array b: {:?}", b);
    println!("Array c: {:?}", c);
    println!("Array months {:?}", months);
}
