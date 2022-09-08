fn arbitrary_mem_add() {
    println!("Creating a raw pointer to an arbitrary memory address");
    let address = 0x012345usize;
    let r = address as *const i32;
}

fn main() {
    arbitrary_mem_add();

    let mut num = 5;

    // Create raw pointers from references
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // Dereferencing raw pointers within an unsafe block
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}
