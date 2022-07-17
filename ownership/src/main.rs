fn main() {
    // "from" method make a mutable variable
    let s1 = String::from("Hello");      // s1 is valid from this point forward
    let s2 = s1;                        // s1 is "moved" to s2

    // println!("{}, world", s1);          // s1 is no more valid
    println!("{}, world", s2);          // But s2 is stil in the scope

    /***    Do a deep copy with clone   ***/
    let s3 = String::from("Clone test");
    let s4 = s3.clone();

    println!("s3 = {}, s4 = {}", s3, s4);
}
