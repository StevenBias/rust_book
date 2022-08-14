pub fn add_two(a: i32) -> i32 {
    a + 2
}
#[cfg(test)]
mod tests {
    use super::*;
#[test]
    fn it_adds_two() {
        let res = 4;
        assert_eq!(res, add_two(2),
        "The result is false, it must be {} instead of {}", res, add_two(2));
    }
}
