fn main() {
    let s1 = String::from("Hello");      // s1 is valid from this point forward
    let s2 = s1;                        // s1 is "moved" to s2

    // println!("{}, world", s1);          // s1 is no more valid
    println!("{}, world", s2);          // But s2 is stil in the scope
}
