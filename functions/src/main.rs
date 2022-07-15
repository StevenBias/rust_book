fn main() {
    another_function(5, 12);
    statement_function();
    println!("The value returned by five function is: {}", five());
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

/*  Statements do not return value
 *
 *  Do not add a semicolon at the end of "x + 1" to keep it as en expression
 *  It becomes a statement otherwise
 */
fn statement_function() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}
