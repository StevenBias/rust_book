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

fn main() {
    arbitrary_mem_add();
    unsafe { raw_pointers();}

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
