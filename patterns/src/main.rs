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

fn pattern_syntax() {
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

fn destrucure_struct() {
    println!("\nDestructuring structs");
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point{x: 3, y: 5};
    // Another way to write the follow instruction is:
    // let Point{a, y}
    let Point{x: a, y: b} = p;

    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

fn ignoring_pattern() {
    println!("\nIgnoring pattern");
    let number = (2, 4, 8, 16, 32);
    match number {
        (first, .., last) => println!("First and last numbers: {}, {}", first, last),
    }
}

fn binding() {
    println!("\n@ Binding");
    enum Message {
        Hello {id: i32},
    }

    let msg = Message::Hello{id: 5};

    match msg {
        Message::Hello{id: id_variable @ 3..=7} => {
            println!("Found an id in range: {}", id_variable)
        },
        Message::Hello{id: 10..=12} => {
            println!("Found an id in another range")
        },
        Message::Hello{ id } => {
            println!("Found some other id: {}", id)
        }
    }
}

fn main() {
    while_let();
    for_loop();
    function_parameters();
    pattern_syntax();
    destrucure_struct();
    ignoring_pattern();
    binding();
}
