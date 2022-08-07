pub fn merge_sort(vector: &Vec<i32>) -> Vec<i32> {
    if vector.len() > 1 {
        let middle = vector.len()/2;

        let left_sorted = merge_sort(&vector[0..middle].to_vec());
        let right_sorted = merge_sort(&vector[middle..].to_vec());
        return merge(&left_sorted , &right_sorted)
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
