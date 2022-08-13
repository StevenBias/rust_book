use std::io;
use std::fs;

fn read_username_from_file() -> Result<String, io::Error> {
    return fs::read_to_string("hello.txt");
}

fn main() {
    let _username = read_username_from_file();

    let _username = match _username {
        Ok(s) => println!("The username is: {}", s),
        Err(e) => panic!("Error: {:?}", e),
    };
}
