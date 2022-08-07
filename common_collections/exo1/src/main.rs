use std::collections::HashMap;
use sort_algo::merge_sort;

fn main() {
    let v = vec![38, 27, 43, 3, 9, 82, 10, 43];
    println!("Vector is: {:?}", v.to_vec());

    let mut mean = 0;
    for i in &v.to_vec() {
        mean += i;
    };
    mean /= &v.len().try_into().unwrap();       // Use try_into and unwrap to avoid v to become Vec<usize> ...
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
    println!("Mode: {:?}", mode);
}
