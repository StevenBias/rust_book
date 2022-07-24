#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum UsState {
    State0,
    State1,
    State2,
    /****/
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("Home value: {:?}", home);
    println!("loopback value: {:?}", loopback);

    // Null values
    let set_number = Some(5);
    let set_string = Some("toto");
    let absent_number: Option<i32> = None;

    println!("set_number value: {:?}", set_number);
    println!("set_string value: {:?}", set_string);
    println!("absent_number value: {:?}", absent_number);

    // The match Control Flow Operator
    let state = UsState::State1;
    let coin = Coin::Quarter(state);
    value_in_cents(coin);

    // Matching with Option<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("The value of six is: {:?}", six);
    println!("The value of none is: {:?}", none);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny     => 1,
        Coin::Nickel    => 5,
        Coin::Dime      => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}
