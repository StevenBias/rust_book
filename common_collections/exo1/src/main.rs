use sort_algo::merge_sort;

fn main() {
    // let v = vec![10, 20, 5];
    let v = vec![38, 27, 43, 3, 9, 82, 10];
    println!("Vector is: {:?}", v.to_vec());

    let mut mean = 0;
    for i in &v.to_vec() {
        mean += i;
    };
    mean /= &v.len().try_into().unwrap();       // Use try_into and unwrap to avoid v to become Vec<usize> ...
    println!("Mean is: {}", mean);

    let sorted_vec = merge_sort(&v);
    println!("Sorted is: {:?}", sorted_vec);
    let median = sorted_vec[sorted_vec.len()/2];
    println!("Median is: {}", median);
}
