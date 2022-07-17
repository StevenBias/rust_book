fn main() {
    /*****************************************/
    /******     Variables in heap       ******/
    /*****************************************/
    //  Strings are variables with unknown size, so it's stored in heap
    //  When the data is "moved", it is no more valid

    // "from" method make a mutable variable
    let s1 = String::from("Hello");      // s1 is valid from this point forward
    let s2 = s1;                        // s1 is "moved" to s2

    // println!("{}, world", s1);          // s1 is no more valid
    println!("{}, world", s2);          // But s2 is stil in the scope

    /***    Do a deep copy with clone   ***/
    let s3 = String::from("Clone test");
    let s4 = s3.clone();

    println!("s3 = {}, s4 = {}", s3, s4);


    /******************************************/
    /******     Variables in stack       ******/
    /******************************************/
    //  All "primitives" types have known size
    //  So they are stored in stack
    //  When they are "moved", the variable is still valid
    //  There is no difference between deep and shallow copying
    //  So no need to use the "clone" method
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);


    /*********************************************/
    /******     Ownership and functions     ******/
    /*********************************************/
    let s = String::from("hello");
    // s comes into scope
    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here
    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to
                   // still use x afterward


    /*********************************************/
    /******     Return values and scope     ******/
    /*********************************************/
    let str1 = gives_ownership();                   // gives_ownership moves its return
                                                    // value into str1

    println!("str1 = {}", str1);

    let str2 = String::from("hello");
        // str2 comes into scope
    let str3 = takes_and_gives_back(str2);          // s2 is moved into
                                                    // takes_and_gives_back, which also
                                                    // moves its return value into str3
    println!("str3 = {}", str3);


    /*********************************************/
    /******     References and borrowing    ******/
    /*********************************************/
    //  Use references
    let mut str4 = String::from("Hello");
    let len = calculate_length(&str4);
    change_string(&mut str4);
    println!("The length of {} is: {}", str4, len);
    println!("The value of str4 is: {}", str4);
} // Here, str3 goes out of scope and is dropped. str2 goes out of scope but was
  // moved, so nothing happens. str1 goes out of scope and is dropped.

fn change_string(s: &mut String) {
    s.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()         // s is immutable, bc it is borrowed by reference
}

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it
    let some_string = String::from("hello");        // some_string comes into scope
    some_string                                     // some_string is returned and
                                                    // moves out to the calling
                                                    // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope
    a_string                                          // a_string is returned and moves out to the calling function
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
