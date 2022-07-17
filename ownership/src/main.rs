fn main() {
    /******     Variables in heap       ******/
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

    /******     Variables in heap       ******/
    //  All "primitives" types have known size
    //  So they are stored in stack
    //  When they are "moved", the variable is still valid
    //  There is no difference between deep and shallow copying
    //  So no need to use the "clone" method
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
    let s = String::from("hello");
    // s comes into scope
    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here
    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to
                   // still use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved,
  // nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
