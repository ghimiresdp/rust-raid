# Rust Raid

[Rust Raid](https://github.com/ghimiresdp/rust-raid) is a repository for Rust
learners and coding challenge seekers.

The repository contains solutions to diverse challenges categorized by different
topics(workspaces). Each workspace contains multiple binaries so that it will be
easier to run specific problem by selecting binaries.

You can run the `cargo run --bin <binary_name>` to run binaries.To run all test cases,
you can run `cargo test` command, or to run specific test, you can run
`cargo test --bin <binary_name>`

```bash
# Example: running binary for huffman encoding
cargo run --bin huffman
cargo test --bin huffman
```

## [1. Data Structures](./data-structures/)

1. **Arrays**
    - [Find the missing number](data-structures/src/ds001_find_missing_number.rs) `cargo run --bin ds001`
    - [Find the length of the longest sub-array with sum K](data-structures/src/ds002_longest_subarray.rs) `cargo run --bin ds002`
2. **Singly Linked Lists**
    - [Add two linked list](./data-structures/src/ds101_linked_list_add.rs) `cargo run --bin ds101`
3. [**Doubly Linked Lists**](./data-structures/src/doubly_linked_list.rs) `cargo run --bin doubly_linked_list`
4. [**Stacks**](./data-structures/src/stack.rs) `cargo run --bin stack`
5. [**Queues**](./data-structures/src/queue.rs) `cargo run --bin queue`
6. [Binary Trees](./data-structures/src/binary_tree.rs) `cargo run --bin binary_tree`
7. Segment Trees
8. Fenwick Trees (Binary Indexed trees)
9. Suffix Trees
10. Trie
11. Disjoint Set

## [2. Algorithms](./algorithms/README.md)

### [2.1. Searching](algorithms/searching/)

1. [Linear Searching](algorithms/searching/linear_search.rs) `cargo run --bin linear_search`
2. [Binary Searching](algorithms/searching/binary_search.rs) `cargo run --bin binary_search`
3. [Depth First Search (DFS)]
4. [Breadth First Search (BFS)]

### [2.2. Sorting](algorithms/sorting/)

1. [bubble sort](algorithms/sorting/bubble_sort.rs) `cargo run --bin bubble_sort`
2. [selection sort](algorithms/sorting/selection_sort.rs) `cargo run --bin selection_sort`
3. [insertion sort](algorithms/sorting/insertion_sort.rs) `cargo run --bin insertion_sort`
4. [quick sort](algorithms/sorting/quick_sort.rs) `cargo run --bin quick_sort`
5. [Merge sort]
6. [heap Sort]
7. [Counting Sort]
8. [Radix Sort]

### [2.3 Greedy and Graph Algorithms](algorithms/greedy/)

1. [Activity Selection]
2. [Huffman Coding](algorithms/greedy/huffman_coding.rs) `cargo run --bin huffman`
3. [Krushkal's algorithm](algorithms/greedy/kruskal.rs) `cargo run --bin kruskal`
4. [Prim's Algorithm]
5. [Dijkstra's Algorithm]
6. [Bellman-Ford Algorithm]
7. [Floyd-Warshall Algorithm]
8. [Topological Sort]
9. [A* Search Algorithm]

## [3. Design Patterns](./design-patterns/README.md)

1. [Singleton Pattern](design-patterns/src/singleton.rs) `cargo run --bin singleton`
2. [Factory Pattern](design-patterns/src/factory.rs) `cargo run --bin factory`
3. [Builder Pattern](design-patterns/src/builder.rs) `cargo run --bin builder`
4. [Decorator Pattern](design-patterns/src/decorator.rs) `cargo run --bin decorator`
5. [Observer Pattern](design-patterns/src/observer.rs) `cargo run --bin observer`
6. [Strategy Pattern](design-patterns/src/strategy.rs) `cargo run --bin strategy`
7. [Command Pattern](design-patterns/src/command.rs) `cargo run --bin command`
8. [Adapter Pattern](design-patterns/src/adapter.rs) `cargo run --bin adapter`

## [4. Problem Solving](problem-solving/README.md)

### [4.1. Basic Problems](problem-solving/basic/)

1. [Practical Number](problem-solving/basic/practical_number.rs)  `cargo run --bin practical_number`
2. [Greatest Common Divisor](problem-solving/basic/gcd.rs) `cargo run --bin gcd`
3. [Median](problem-solving/basic/median.rs) `cargo run --bin median`
4. [Reverse digits of the integer](problem-solving/basic/reverse_integer.rs) `cargo run --bin reverse_integer`
5. [List Comprehension](problem-solving/basic/comprehension.rs) `cargo run --bin comprehension`
6. [Linear Regression Model](problem-solving/basic/linear_regression.rs) `cargo run --bin linear_regression`
7. [Matrix Multiplication Model](problem-solving/basic/matrix_multiplication.rs) `cargo run --bin matrix_multiplication`
8. [Color Converter](problem-solving/basic/color_converter.rs) `cargo run --bin color_converter`

### [4.2. Dynamic Programming](problem-solving/dp/)

1. [List group by consecutive numbers](problem-solving/dp/consecutive_groups.rs) `cargo run --bin consecutive_groups`
2. [Find the length of the longest substring with maximum 2 repetition](problem-solving/dp/repeat.rs)`cargo run --bin repeat`
3. [Find the index of 2 numbers in an array whose sum equals to the provided target](problem-solving/dp/two_sum.rs) `cargo run --bin two_sum`
4. [Minimize the Sum from an array](problem-solving/dp/minimize_sum.rs) `cargo run --bin minimize_sum`
5. [Fibonacci Series](problem-solving/dp/fibonacci.rs) `cargo run --bin fibonacci`
6. [Longest Common Subsequence](problem-solving/dp/longest_common_subsequence.rs) `cargo run --bin lcs`
7. [Coin Change Problem](problem-solving/dp/coin_change.rs) `cargo run --bin coin_change`
8. [Palindrome Partition](problem-solving/dp/palindrome_partition.rs) `cargo run --bin palindrome_partition`

## [5. Projects](./projects/)

1. [Python's `Pandas` like dataframe container](projects/pandas/README.md) `cargo run --bin pandas`
2. [`ruscrypt` basic encryption](projects/ruscrypt/README.md) `cargo run --bin ruscrypt`

> **Note**: Topics that do not contain hyperlinks are work in progress and will
> be updated as soon as the solution gets completed.
>
> You can also create a PR with solution/enhancement to each topics.
>
## Running binaries

To run any binary, you can run the command `cargo run --bin <bin_name>`

Example:

```shell
cargo run --bin practical_number
```

> **Note:** Binary names might not always be the name of the file. Sometimes, a
> shorter version of the solution name is used to make easier to type. You can
> see the name of binary in the respective `README.md` file or the `docstring`
> of the respective solution.
> some examples of shorter version of solution name are as follows:
>
> - The binary for `huffman_coding.rs` is just `huffman`.

## Testing

There are test cases for each functions/challenges which will be beneficial
for you to learn testing as well as test programs for errors.

To test programs, you can run `cargo test` command.

Example:

```shell
cargo test

# alternatively, to test individual binary, you can run
cargo test --bin your_program_name
```
