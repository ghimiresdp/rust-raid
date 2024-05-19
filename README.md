# rust-challenges

This is a repository for Rust learners as well as coding challenge seekers.

The repository contains various workspace members with multiple binaries
The binaries are specified in the respective `cargo.toml` file.

## List of workspace members

_Note: Topics that do not contain hyperlinks are work in progress. If you want to contribute to any topic, you can create a PR._

### [1. `_lib` (See contents)](_lib/) : Contains common methods used by all packages

### [2. Data Structures](./data-structures/)

#### 2.1. **Basic Data Structures**

1. **Arrays**
      - [Find the missing number](./data-structures/ds001_find_missing_number.rs) `cargo run --bin ds001`
2. **Singly Linked Lists**
3. **Doubly Linked Lists**
4. **Stacks**
5. **Queues**

#### 2.2. **Complex Data Structures**

      1. [Binary Trees]
      2. [Heaps]
      3. [Hash tables]
      4. [Graphs]

#### 2.3. **Advanced Data Structures**

1. Segment Trees
2. Fenwick Trees (Binary Indexed trees)
3. Suffix Trees
4. Trie
5. Disjoint Set

### [3. Algorithms](./algorithms/)

#### [3.1. Searching]

1. [Linear Searching](dsa/src/searching/linear_search.rs) `cargo run --bin linear_search`
2. [Binary Searching](dsa/src/searching/binary_search.rs) `cargo run --bin binary_search`
3. [Depth First Search (DFS)]
4. [Breadth First Search (BFS)]

#### [3.2. Sorting]

1. [bubble sort](dsa/src/sorting/bubble_sort.rs) `cargo run --bin bubble_sort`
2. [selection sort](dsa/src/sorting/selection_sort.rs) `cargo run --bin selection_sort`
3. [insertion sort](dsa/src/sorting/insertion_sort.rs) `cargo run --bin insertion_sort`
4. [quick sort](dsa/src/sorting/quick_sort.rs) `cargo run --bin quick_sort`
5. [Merge sort]
6. [heap Sort]
7. [Counting Sort]
8. [Radix Sort]

#### [3.3 Greedy Algorithms]

1. [Activity Selection]
2. [Huffman Coding]
3. [Krushkal's algorithm]
4. [Prim's Algorithm]

#### [3.4. Graph Algorithms]

1. [Dijkstra's Algorithm]
2. [Bellman-Ford Algorithm]
3. [Floyd-Warshall Algorithm]
4. [Topological Sort]
5. [A* Search Algorithm]

### 4. [Design Patterns](./design_patterns/)

1. [Singleton Pattern](design_patterns/src/singleton.rs) `cargo run --bin singleton`
2. [Factory Pattern](design_patterns/src/factory.rs) `cargo run --bin factory`
3. [Builder Pattern](design_patterns/src/builder.rs) `cargo run --bin builder`
4. [Decorator Pattern](design_patterns/src/decorator.rs) `cargo run --bin decorator`
5. [Observer Pattern](design_patterns/src/observer.rs) `cargo run --bin observer`
6. [Strategy Pattern](design_patterns/src/strategy.rs) `cargo run --bin strategy`
7. [Command Pattern](design_patterns/src/command.rs) `cargo run --bin command`
8. [Adapter Pattern](design_patterns/src/adapter.rs) `cargo run --bin adapter`

### 5. [Design Techniques]

1. [Backtracking]
2. [Divide and Conquer]
3. [Branch and Bound]

### 6. [Complexity Analysis](./complexity_analysis/)

1. Time Complexity analysis
2. Space Complexity analysis

### 7. [Dynamic Programming](./dynamic_programming/)

1. [Fibonacci Series]
2. [Longest Commmon Subsequence]
3. [Knapsack Problem]
4. [Matrix Multiplication]

### 8. [Problem Solving](./problem_solving/)

### 9. [Projects](./projects/)

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
