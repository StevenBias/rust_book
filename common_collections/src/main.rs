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
}
