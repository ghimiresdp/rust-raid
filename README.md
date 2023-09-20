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

## List of packages in this repository

1. [`_lib` (See contents)](_lib/) : Contains common methods used by all other packages

2. [`basic` Challenges](challenge_basic/) : Contains Basic challenges

3. [`mid`-level Challenges](challenge_mid/) : Contains Intermediate challenges

4. [`pro` Challenges](challenge_pro/) : Contains Pro challenges

5. [`design_patterns` (See contents)](design_patterns/): Contains Design Pattern examples
   1. [Singleton Pattern](design_patterns/src/singleton.rs)
   2. [Factory Pattern](design_patterns/src/factory.rs)
   3. [Builder Pattern](design_patterns/src/builder.rs)
   4. [Decorator Pattern](design_patterns/src/decorator.rs)
   5. [Observer Pattern](design_patterns/src/observer.rs)
   6. [Strategy Pattern](design_patterns/src/strategy.rs)
   7. [Command Pattern](design_patterns/src/command.rs)
   8. [Adapter Pattern](design_patterns/src/adapter.rs)

6. [Data Structure and Algorithms](dsa/): Contains Data Structures and Algorithm examples
   1. Searching
      1. [Linear Searching](dsa/src/searching/linear_search.rs)
      2. [Binary Searching](dsa/src/searching/binary_search.rs)
   2. Sorting
      1. [bubble sort](dsa/src/sorting/bubble_sort.rs)
      2. [selection sort](dsa/src/sorting/selection_sort.rs)
      3. [insertion sort](dsa/src/sorting/insertion_sort.rs)
      4. [quick sort](dsa/src/sorting/quick_sort.rs)

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
