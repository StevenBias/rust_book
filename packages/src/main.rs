use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IoResult;

// The foolowing is equivalent to:
// use std::io;
// use std::cmp::Ordering;
// use std::{io, cmp::Ordering};

// The foolowing is equivalent to:
// use std::io;
// use std::io::Write;
use std::io::{self, Write};

// Bring all public items from std::collections
use std::collections::*;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

fn function1() -> Result {
    /**/
    Ok(())
}

fn function2() -> IoResult<()> {
    /**/
    Ok(())
}
