# Cheatsheet for rust book


## Common Cargo commands
### Build a prject
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
