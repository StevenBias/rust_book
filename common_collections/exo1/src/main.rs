fn main() {
    let v = vec![10, 20];
    let mut mean = 0;
    for i in &v {
        mean += i;
    };
    mean /= v.len();
    println!("Mean is: {}", mean);
}
