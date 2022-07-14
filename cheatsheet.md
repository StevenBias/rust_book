# Cheatsheet for cargo


## Common Cargo commands
### Build a prject
cargo build

// Check code to be sure it complies, does not create an executable
cargo check


# Build and run a project
cargo run

# Build for release
The default directory of the build result is in "target/debug" and the the compulation is not optimized.

To build a release with optimization, the command is:
cargo build --release
The result will be in "target/release"
