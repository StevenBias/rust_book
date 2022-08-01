fn main() {
    let v = vec![10, 20, 5, 15, 35, 23];
    println!("Vector is: {:?}", v);

    // let mut mean = 0;
    // for i in &v {
    //     mean += i;
    // };
    // mean /= &v.len();
    // println!("Mean is: {}", mean);

    let m = median(&v);
    println!("Median is: {}", m);
}

fn median(v: &Vec<i32>) -> i32 {
    let res = merge_sort(&v.to_vec(), 0, v.len().try_into().unwrap());
    println!("Sorted is: {:?}", res);
    res[res.len()/2]
}

fn merge_sort(vector: &Vec<i32>, left: i32, right: i32) -> Vec<i32> {
    println!("Left: {:?}", left);
    println!("Right: {:?}", right);
    if left < right && vector.len() > 1 {
        let middle = left + (right - left) / 2;
        let mut v_left: Vec<i32> = Vec::new();
        // v_left.push(vector[0 as usize]);
        for i in 0..middle {
            v_left.push(vector[i as usize]);
        }
        println!("Sorted left is: {:?}", v_left);
        v_left = merge_sort(&v_left, left, middle.try_into().unwrap());
        let mut v_right: Vec<i32> = Vec::new();
        // v_right.push(vector[middle as usize]);
        for i in middle-1..(right-1).try_into().unwrap() {
            v_right.push(vector[i as usize]);
        }
        println!("Sorted right is: {:?}", v_right);
        v_right = merge_sort(&v_right, (middle+1).try_into().unwrap(), right);
        // v_left.append(&mut v_right);
        // let v: Vec<i32> = &v_left;
        // println!("Sorted append is: {:?}", v);
        merge(&v_left, &v_right)
    }
    else {
        vector.to_vec()
    }
}

fn merge(vector_left: &Vec<i32>, vector_right: &Vec<i32>) -> Vec<i32> {
    // if vector.len() == 0 { return Vec::new() }
    let mut ind_left = 0;//left as usize;
    let mut ind_right = 0;//(left + (right - left) /2) as usize;
    // let middle = vector.len()/2;
    let mut merged: Vec<i32> = Vec::new();
    // Create two distinct sub-slices from a slice:
    // let (vector_left, vector_right) = vector.split_at(middle.try_into().unwrap());     // Use try_into().unwrap() to convert a 'u32' to a 'usize' and panic if the converted value doesn't fit!
    // println!("vector_left is: {:?}", vector_left);
    // println!("vector_right is: {:?}", vector_right);

    while ind_left < vector_left.len().try_into().unwrap()
        && ind_right < vector_right.len().try_into().unwrap() {
        if vector_left[ind_left] < vector_right[ind_right] {
            merged.push(vector_left[ind_left]);
            println!("merged1 is: {:?}", merged);
            ind_left += 1;
        }
        else {
            merged.push(vector_right[ind_right]);
            println!("merged2 is: {:?}", merged);
            ind_right += 1;
        }
    }

    if ind_left < vector_left.len().try_into().unwrap() {
        while ind_left < vector_left.len().try_into().unwrap() {
            merged.push(vector_left[ind_left]);
            println!("merged3 is: {:?}", merged);
            ind_left += 1;
        }
    }

    if ind_right < vector_right.len().try_into().unwrap() {
        while ind_right < vector_right.len().try_into().unwrap() {
            merged.push(vector_right[ind_right]);
            println!("merged4 is: {:?}", merged);
            ind_right += 1;
        }
    }

    merged
}
