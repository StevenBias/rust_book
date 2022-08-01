fn main() {
    let v = vec![10, 20, 5];

    let mut mean = 0;
    for i in &v {
        mean += i;
    };
    mean /= v.len();
    println!("Mean is: {}", mean);
}

fn mergeSort(vector: &mut Vec<i32>, left: u32, right: u32) {
    if left > right {
        let middle = left + (left - right)/2;
        mergeSort(vector, left, middle);
        mergeSort(vector, middle+1, right);
        merge(vector, left, middle, right);
    }
}

fn merge(vector: &mut Vec<i32>, left: u32, middle: u32, right: u32) {
    let mut index = left;
    let mut indLeft = 0;
    let mut indRight = 0;
    // Create two distinct sub-slices from a slice:
    let (vectorLeft, vectorRight) = vector.split_at_mut(middle.try_into().unwrap());     // Use try_into().unwrap() to convert a 'u32' to a 'usize' and panic if the converted value doesn't fit!

    while indLeft < middle {
        if &vectorLeft[indLeft.try_into().unwrap()] <= &vectorRight[indRight.try_into().unwrap()] {
            &vector[index.try_into().unwrap()] = &vectorLeft[indLeft.try_into().unwrap()];
        }
    }
}
