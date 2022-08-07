fn main() {
    let v = vec![10, 20, 5];
    println!("Vector is: {:?}", v.to_vec());

    let mut mean = 0;
    for i in &v.to_vec() {
        mean += i;
    };
    mean /= &v.len().try_into().unwrap();       // Use try_into and unwrap to avoid v to become Vec<usize> ...
    println!("Mean is: {}", mean);

    let m = median(&v);
    println!("Median is: {}", m);
}

fn median(v: &Vec<i32>) -> i32 {
    let res = merge_sort(&v.to_vec());
    println!("Sorted is: {:?}", res);
    res[res.len()/2]
}

fn merge_sort(vector: &Vec<i32>) -> Vec<i32> {
    if vector.len() > 1 {
        let middle = vector.len()/2;
        let mut v_left: Vec<i32> = Vec::new();
        for i in 0..middle as i32 {
            v_left.push(vector[(i) as usize]);
        }
        v_left = merge_sort(&v_left);
        let mut v_right: Vec<i32> = Vec::new();
        for i in middle..vector.len() {
            v_right.push(vector[(i) as usize]);
        }
        v_right = merge_sort(&v_right);
        merge(&v_left, &v_right)
    }
    else {
        vector.to_vec()
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
    merged
}
