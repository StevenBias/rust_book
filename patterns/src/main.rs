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
        println!("{} is at index {}", value, index);
    }
}

fn print_coordonates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn function_parameters() {
    println!("\nprint_coordonates");
    let point = (3, 5);
    print_coordonates(&point);
}

fn main() {
    while_let();
    for_loop();
    function_parameters();

    println!("");
    let x = 1;
    match x {
        // Multiple pattern
        1 | 2 => println!("1 or 2"),
        3     => println!("three"),
        _     => println!("anything"),
    }


    println!("");
    let x = 5;
    match x {
        // Matching range
        // '...' range pattern is deprecated
        1 ..= 5 => println!("one through five"),
        _       => println!("something else"),
    }
}
