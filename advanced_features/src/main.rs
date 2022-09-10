use std::slice;

fn arbitrary_mem_add() {
    println!("Creating a raw pointer to an arbitrary memory address");
    let address = 0x012345usize;
    let r = address as *mut i32;

    // We have no guarantee this code is safe
    // bc we don't own the memory at this arbitrary location
    let slice: &[i32] = unsafe {
        slice::from_raw_parts_mut(r, 10000)
    };
}

unsafe fn raw_pointers() {
    let mut num = 5;

    // Create raw pointers from references
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // Dereferencing raw pointers within an unsafe block
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// Immutable global variable
static HELLO_WORLD: &str = "Hello, world!";

// Mutable global variable
// It is unsafe
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

unsafe trait Foo {}

unsafe impl Foo for i32 {}

fn unsafe_function() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);

    // Any code that reads or writes from COUNTER must be within an unsafe block.
    // Having multiple threads access COUNTER would likely result in data races.
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

fn advanced_traits() {
    println!("\nAdvanced traits");
    use std::ops::Add;

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    assert_eq!(Point {x: 1, y: 0} + Point {x: 2, y: 3},
               Point {x: 3, y: 3});

    struct Millimeters(u32);
    struct Meters(u32);

    // Specify impl Add<Meters> to set the value of the Right Hand Side type parameter instead of
    // using the default of self
    impl Add<Meters> for Millimeters {
        type Output = Millimeters;
        
        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }
}

fn disambiguation() {
    println!("\nCalling methods with the same name");

    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }

    let person = Human;
    println!("Pilot fly method:");
    Pilot::fly(&person);
    println!("\nWizard fly method:");
    Wizard::fly(&person);
    println!("\nHuman fly method:");
    person.fly();

    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

fn main() {
    arbitrary_mem_add();
    unsafe { raw_pointers();}
    unsafe_function();

    advanced_traits();
    disambiguation();
}
