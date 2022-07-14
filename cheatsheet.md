# Cheatsheet for rust book


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

# Update crate
The update of the cargo will update the "Cargo.lock" file.
A cargo command allow you to check for minor new version (0.3.11 -> 0.3.15 for instance).
This command is:
cargo update

But if you want to update newer version, you have to update the "Cargo.toml" file.
