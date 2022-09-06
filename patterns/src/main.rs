fn while_let() {
    println!("\nwhile_let");
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn for_loop() {
    println!("\nfor_loop");
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{}", value);
    }
}

fn main() {
    while_let();
    for_loop();
}
