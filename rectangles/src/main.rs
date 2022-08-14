#[derive(Debug)]
struct Rectangle {
    width:      u32,
    height:     u32
}
#[cfg(test)]
mod tests {
    // Add super to import Rectangle struct in the module
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { height: 8, width: 7 };
        let smaller = Rectangle { height: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { height: 8, width: 7 };
        let smaller = Rectangle { height: 5, width: 1 };

        assert!(!smaller.can_hold(&larger));
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size}
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50};
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    let sq = Rectangle::square(5);

    println!("rect1 is: {:?}", rect1);
    println!("rect2 is: {:?}", rect2);
    println!("rect3 is: {:?}", rect3);
    println!("sq is: {:?}", sq);

    println!(
"The area of the rectangle is {} square pixels.",
rect1.area()
);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
