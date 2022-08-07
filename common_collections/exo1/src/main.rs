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

fn merge_sort(vector: &Vec<i32>) -> Vec<i32> {
    if vector.len() > 1 {
        let middle = vector.len()/2;

        let v_left: Vec<i32> = (&vector[0..middle]).to_vec();
        let v_left_sorted = merge_sort(&v_left);

        let v_right: Vec<i32> = (&vector[middle..]).to_vec();
        let v_right_sorted = merge_sort(&v_right);
        return merge(&v_left_sorted , &v_right_sorted )
    }
    else {
        return vector.to_vec()
    }
}

fn merge(vector_left: &Vec<i32>, vector_right: &Vec<i32>) -> Vec<i32> {
    let mut ind_left = 0;
    let mut ind_right = 0;
    let mut merged: Vec<i32> = Vec::new();

    while ind_left < vector_left.len().try_into().unwrap()
        && ind_right < vector_right.len().try_into().unwrap() {
        if vector_left[ind_left] < vector_right[ind_right] {
            merged.push(vector_left[ind_left]);
            ind_left += 1;
        }
        else {
            merged.push(vector_right[ind_right]);
            ind_right += 1;
        }
    }

    if ind_left < vector_left.len().try_into().unwrap() {
        while ind_left < vector_left.len().try_into().unwrap() {
            merged.push(vector_left[ind_left]);
            ind_left += 1;
        }
    }
    if ind_right < vector_right.len().try_into().unwrap() {
        while ind_right < vector_right.len().try_into().unwrap() {
            merged.push(vector_right[ind_right]);
            ind_right += 1;
        }
    }
    return merged
}
