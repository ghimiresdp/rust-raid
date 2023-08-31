# rust-challenges

This is a repository for Rust learners as well as coding challenge seekers.

The repository contains 4 different packages with multiple binaries inside them.
The binaries are specified in the respective `cargo.toml` file.

```toml
[dependencies]

[[bin]]
name = "minimize_sum"
path = "src/minimize_sum.rs"

[[bin]]
name = "practical_number"
path = "src/practical_number.rs"
```

## Table of Contents

1. [Package: `_lib` (See contents)](_lib/)
2. [Package: `basic` (See contents)](basic/)
3. [Package: `mid` (See contents)](mid/)
4. [Package: `pro` (See contents)](pro/)

## Running binaries

To run any binary, you can run the command `cargo run --bin <bin_name>`

Example:

```shell
cargo run --bin practical_number
```

## Testing

I have added test cases for each functions/challenges which will be beneficial
for you to learn testing as well as test programs for errors.

To test programs, you can run `cargo test` command.

Example:

```shell
cargo test

# alternatively, to test individual binary, you can run
cargo test --bin your_program_name
```
