# rust-challenges

This is a repository for Rust learners as well as coding challenge seekers.

The repository contains various workspace members with multiple binaries
The binaries are specified in the respective `cargo.toml` file.

## List of workspace members

> _Note: Topics that do not contain hyperlinks are work in progress. If you want to contribute to any topic, you can create a PR._
>
> To run/test any binary, you can use `cargo run` and `cargo test` commands
>
> - Example 1: `cargo run --bin fib`
> - Example 2: `cargo test --bin fib`

### [`_lib`](_lib/)

This module contains common methods used by various packages.

### [1. Data Structures](./data-structures/)

#### 1.1. **Basic Data Structures**

1. **Arrays**
      - [Find the missing number](./data-structures/ds001_find_missing_number.rs) `cargo run --bin ds001`
      - [Find the length of the longest subarray with sum K](./data-structures/ds002_longest_subarray.rs) `cargo run --bin ds002`
2. **Singly Linked Lists**
      - [Add two linked list](./data-structures/ds101_linked_list_add.rs) `cargo run --bin ds101`
3. **Doubly Linked Lists**
4. **Stacks**
5. **Queues**

#### 1.2. **Complex Data Structures**

      1. [Binary Trees]
      2. [Heaps]
      3. [Hash tables]
      4. [Graphs]

#### 1.3. **Advanced Data Structures**

1. Segment Trees
2. Fenwick Trees (Binary Indexed trees)
3. Suffix Trees
4. Trie
5. Disjoint Set

### [2. Algorithms](./algorithms/)

#### [2.1. Searching]

1. [Linear Searching](dsa/src/searching/linear_search.rs) `cargo run --bin linear_search`
2. [Binary Searching](dsa/src/searching/binary_search.rs) `cargo run --bin binary_search`
3. [Depth First Search (DFS)]
4. [Breadth First Search (BFS)]

#### [2.2. Sorting]

1. [bubble sort](dsa/src/sorting/bubble_sort.rs) `cargo run --bin bubble_sort`
2. [selection sort](dsa/src/sorting/selection_sort.rs) `cargo run --bin selection_sort`
3. [insertion sort](dsa/src/sorting/insertion_sort.rs) `cargo run --bin insertion_sort`
4. [quick sort](dsa/src/sorting/quick_sort.rs) `cargo run --bin quick_sort`
5. [Merge sort]
6. [heap Sort]
7. [Counting Sort]
8. [Radix Sort]

#### [2.3 Greedy Algorithms]

1. [Activity Selection]
2. [Huffman Coding]
3. [Krushkal's algorithm]
4. [Prim's Algorithm]

#### [2.4. Graph Algorithms]

1. [Dijkstra's Algorithm]
2. [Bellman-Ford Algorithm]
3. [Floyd-Warshall Algorithm]
4. [Topological Sort]
5. [A* Search Algorithm]

### [3. Design Patterns](./design-patterns/)

1. [Singleton Pattern](design-patterns/src/singleton.rs) `cargo run --bin singleton`
2. [Factory Pattern](design-patterns/src/factory.rs) `cargo run --bin factory`
3. [Builder Pattern](design-patterns/src/builder.rs) `cargo run --bin builder`
4. [Decorator Pattern](design-patterns/src/decorator.rs) `cargo run --bin decorator`
5. [Observer Pattern](design-patterns/src/observer.rs) `cargo run --bin observer`
6. [Strategy Pattern](design-patterns/src/strategy.rs) `cargo run --bin strategy`
7. [Command Pattern](design-patterns/src/command.rs) `cargo run --bin command`
8. [Adapter Pattern](design-patterns/src/adapter.rs) `cargo run --bin adapter`

### [4. Design Techniques]

1. [Backtracking]
2. [Divide and Conquer]
3. [Branch and Bound]

### [5. Complexity Analysis](./complexity-analysis/)

1. Time Complexity analysis
2. Space Complexity analysis

### [6. Dynamic Programming](./dynamic-programming/)

1. [Fibonacci Series](./dynamic-programming/src/dp001_fibonacci.rs)
    - run:  `cargo run --bin fib`
    - test:  `cargo test --bin fib`
2. [Longest Commmon Subsequence](./dynamic-programming/src/dp002_lcs.rs)
    - run:  `cargo run --bin lcs`
    - test:  `cargo test --bin lcs`
3. [Knapsack Problem]
4. [Matrix Multiplication]

### [7. Problem Solving](./problem-solving/)

### [8. Projects](./projects/)

## Running binaries

To run any binary, you can run the command `cargo run --bin <bin_name>`

Example:

```shell
cargo run --bin practical_number
```

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
