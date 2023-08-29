# rust-challenges

This is a repository for Rust learners.

The basic file structure for the project is as follows:

```shell
workspace
├── Cargo.lock
├── Cargo.toml
├── core
│   ├── Cargo.toml
│   └── src
│       ├── fib.rs
│       ├── minimize_sum.rs
│       └── practical_number.rs
├── lib
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── web
    ├── Cargo.toml
    └── src
        └── main.rs
```

Each `cargo.toml` file contains multiple entries for binaries so that smaller challenges do not need new project and `cargo.toml` file.

example:

```toml
[dependencies]

[[bin]]
name = "minimize_sum"
path = "src/minimize_sum.rs"

[[bin]]
name = "practical_number"
path = "src/practical_number.rs"
```

To run any binary, you can run the following command

```shell
cargo run --bin binary_name
```

To test programs, you can run the following command

```shell
cargo test

# alternatively, to test individual binary, you can run
cargo test --bin your_program_name
```
