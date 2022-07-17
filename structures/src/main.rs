struct User {
    username:       String,
    email:          String,
    sign_in_count:  u64,
    active:         bool
}

fn main() {
    let user1 = User {
        username:       String::from("Toto"),
        email:          String::from("abc@email.com"),
        sign_in_count:  1,
        active:         true
    };

    let user2 = User {
        username:       String::from("Tata"),
        email:          String::from("truc@email.com"),
        ..user1
    };

    let user3 = build_user(String::from("machin@email.com"), String::from("titi"));

    println!("Username 1: {}", user1.username);
    println!("Email 1: {}", user1.email);
    println!("Sign in count 1: {}", user1.sign_in_count);
    println!("Active 1: {}\n", user1.active);

    println!("Username 2: {}", user2.username);
    println!("Email 2: {}", user2.email);
    println!("Sign in count 2: {}", user2.sign_in_count);
    println!("Active 2: {}\n", user2.active);

    println!("Username 3: {}", user3.username);
    println!("Email 3: {}", user3.email);
    println!("Sign in count 3: {}", user3.sign_in_count);
    println!("Active 3: {}\n", user3.active);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: false,
        sign_in_count: 1
    }
}
