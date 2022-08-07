use std::io;
use std::collections::HashMap;
use sort_algo::merge_sort;

fn main() {
    println!("Input the size of value you want to test: ");
    let size = scan_number();

    println!("Please enter the {:?} values:", size);
    let mut v:Vec<i32> = Vec::new();
    let mut index = 0;
    while index < size {
        let number = scan_number();
        v.push(number);
        index += 1;
    }
    println!("Vector is: {:?}", v.to_vec());

    let mut sum = 0;
    for i in &v.to_vec() {
        sum += i;
    };
    let mean: f64 = sum as f64/ size as f64;
    println!("Mean is: {}", mean);

    let sorted_vec = merge_sort(&v);
    let median = sorted_vec[sorted_vec.len()/2];
    println!("Median is: {}", median);

    let mut map = HashMap::new();
    let mut max: i32 = 0;
    let mut mode = &v[0];
    for i in &v {
        let count = map.entry(i).or_insert(0);
        *count += 1;
        if *count > max {
            max = *count;
            mode = i;
        }
    }
    println!("Mode: {}", mode);
}

fn scan_number() -> i32 {
    let mut number = String::new();
    io::stdin().read_line(&mut number)
        .expect("Failed to read line");

    let number: i32 = number.trim().parse()
        .expect("Please type a number");
    return number
}
