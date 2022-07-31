fn main() {
    let mut v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3, 4, 5];
    // v2 is immutable
    // v2.push(6);

    let index = 4;
    let third: &i32 = &v2[index];

    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);

    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
    println!("v1[{}]: {}", index, third);

    match v1.get(index) {
        Some(third) => println!("The element {} of v1 is: {}", index+1, third),
        None => println!("There is no element at the place {} of v1", index+1)
    }

    for i in &mut v1 {
        *i *= 10;
    }
    println!("v1: {:?}", v1);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("row: {:?}", row);


    /****   Strings     ****/
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);
    println!("s is: {}", s);
    println!("s2 is: {}", s2);

    let hello = String::from("Hello, ");
    let world = String::from("world!");
    // The +operator signature looks like: fn add(seld, s: &str) -> String
    let s3 = hello + &world;    // hello has been moved here and can no longer be used
    println!("s3 is: {}", s3);

    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");
    // The format! macro doesn't take ownership of its parameters, unlike +operator
    let s7 = format!("{}-{}-{}", s4, s5, s6);
    println!("s7 is: {}", s7);
}
