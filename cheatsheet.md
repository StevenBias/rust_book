# Cheatsheet for rust book


## Common Cargo commands
### Build a project
```
cargo build
```

Check code to be sure it complies, does not create an executable
```
cargo check
```


### Build and run a project
```
cargo run
```

#### Build and run a project with an environment variable
For instance with variable named "ENV_VAR" equal to 1:
```
ENV_VAR=1 cargo run
```

### Automated tests
#### Run all tests
```
cargo test
```
By default, tests run in parrallel with several threads.\
To run tests consecutively, use only one thread:
```
cargo test -- --test-threads=1
```
#### Showing function output
By default, if the test is ok, no output is printed.\
To show print:
```
cargo test --nocapture
```

#### Run only ignored tests
```
cargo test -- --ignored
```

### Build for release
The default directory of the build result is in "target/debug" and the the compulation is not optimized.

To build a release with optimization, the command is:
```
cargo build --release
```
The result will be in "target/release"

### Update crate
The update of the cargo will update the "Cargo.lock" file.
A cargo command allow you to check for minor new version (0.3.11 -> 0.3.15 for instance).
This command is:
```
cargo update
```

But if you want to update newer version, you have to update the "Cargo.toml" file.


### Module System
**Packages** A Cargo feature that lets you build, test, and share crates

**Crates** A tree of modules that produces a library or executable

**Modules** and use Let you control the organization, scope, and privacy
of paths

**Paths** A way of naming an item, such as a struct, function, or module


### Use panic backtrace
To get a backtrace of an error:
```
RUST_BACKTRACE=1 cargo run
```


## Smart pointers
The three most common smart pointers are:
- **Box\<T\>** for allocating values on the heap
- **Rc\<T\>** , a reference counting type that enables multiple ownership
- **Ref\<T\>** and **RefMut\<T\>** , accessed through **RefCell\<T\>** , a type that enforces the borrowing rules at runtime instead of compile time

### Box\<t\>
Boxes are most often used in these situations:
- When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
- When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
- When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

