pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than 1, got {}.", value);
        } else  if value > 100 {
            panic!("Guess value must be less 100, got {}.", value);
        } else {
            Guess {
                value
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
#[test]
#[should_panic(expected = "Guess value must be greater than 1, got -1.")]
    fn less_than_1() {
        Guess::new(-1);
    }
#[test]
#[should_panic(expected = "Guess value must be less 100, got 200.")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
