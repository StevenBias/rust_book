use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    return Ok(s);
}

fn main() {
    let _f = read_username_from_file();

    let _f = match _f {
        Ok(file) => file,
        Err(e) => panic!("Error: {:?}", e),
    };
}
