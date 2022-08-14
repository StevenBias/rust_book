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
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
        let mut i=0;
        while i < 100000000 {
            i += 1;
        }
    }
}
