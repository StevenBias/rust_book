use std::io;                // Input io library

fn main() {
    println!("Input a number to get its fibonacci value: ");
    let mut number = String::new();
    io::stdin().read_line(&mut number)
        .expect("Failed to read line");

    let number: i32 = number.trim().parse()
        .expect("Please type a number");

    println!("The result is: {:?}", fibo(number));
}

fn fibo(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        let mut res = 1;
        let mut fibo_1 = 0;
        for _element in (2..n+1).rev() {
            let temp = res;
            res += fibo_1;
            fibo_1 = temp;
        }
        res
    }
}
