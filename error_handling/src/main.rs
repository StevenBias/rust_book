use std::io;
use std::fs;

use aggregator::{Summary, Tweet};

fn read_username_from_file() -> Result<String, io::Error> {
    Ok(fs::read_to_string("hello.txt")?)
}

fn main() {
    let _username = read_username_from_file();

    let _username = match _username {
        Ok(s) => println!("The username is: {}", s),
        Err(e) => panic!("Error: {:?}", e),
    };

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}
