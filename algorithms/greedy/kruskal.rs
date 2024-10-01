/// Kruskal's Algorithm
///
/// Kruskal's Algorithm is a minimum spanning tree algorithm also known as MST
/// algorithm. it takes a graph as input and finds the subset of the edges of
/// that graph which
/// - forms a tree that includes every vertex
/// - has minimum sum of weights among all the trees that can be formed from
/// - the graph.
///
/// It falls under the greedy algorithm.
///
/// ```
///              8         7
///        (A)-------(B)-------(C)
///       / |         | \       |  \
///     4/  |        2|  \      |   \ 9
///     /   |         |   \     |    \
///  (H)    11       (I)   \4   |14  (D)
///     \   |      /  |     \   |    /
///     8\  |  7 /   6|      \  |   /10
///       \ |  /      |       \ |  /
///        (G)-------(F)-------(E)
///              1        2
///```
/// In the above figure, there are 9 vertices and 14 edes.
/// so the minimum spanning tree(MST) will be 9 -1 = 8 edges.
///
/// after finding the mst, the graph will be as follows:
/// ```
///                        7
///        (A)       (B)-------(C)
///       /           | \          \
///     4/           2|  \          \ 9
///     /             |   \          \
///  (H)             (I)   \4        (D)
///     \                   \
///     8\                   \
///       \                   \
///        (G)-------(F)-------(E)
///              1         2
///
/// steps:
/// 1. sort all the edges from low weight to high
/// 2. Take the edge with the lowest weight and add it to the spanning tree.
///    If the edge creates a cycle then reject the edge.
/// 3. keep adding edges until we reach all vertices.
///```
use std::{collections::HashMap, fmt::Debug};

/// # Edge
///
/// This is a data structure that stores information about the source node,
/// destination node, and weight to travel to the destination node.
/// each point in the graph. Eg: A, B, etc is a vertex.one vertex connects to another

type Edge = (char, char, usize); // (from, to, weight)

fn find_parent(visited: &HashMap<char, char>, vertex: char) -> char {
    print!("⛔ {vertex} -> ");
    if visited.contains_key(&vertex) {
        return find_parent(visited, *visited.get(&vertex).unwrap());
    }
    return vertex;
}

fn rank(a: char, b: char) -> (char, char) {
    if a > b {
        return (a, b);
    } else {
        return (b, a);
    }
}

struct Graph {
    edges: Vec<Edge>,
}
impl Graph {
    fn new(edges: Vec<(char, char, usize)>) -> Self {
        Self { edges }
    }

    fn sort_edges(&mut self) {
        self.edges.sort_by(|a, b| a.2.cmp(&b.2));
    }

    fn find_mst(&mut self) -> Vec<Edge> {
        // let mut graph = Graph::new(vec![]);
        let mut edges = Vec::new();
        let mut visited: HashMap<char, char> = HashMap::new();

        // Step 1: sort all the edges by weight
        self.sort_edges();
        for (a, b, weight) in self.edges.iter() {
            let (a, b) = rank(*a, *b);
            let parent_a = find_parent(&visited, a);
            println!("{parent_a}");
            let parent_b = find_parent(&visited, b);
            println!("{parent_b}");
            if parent_a != parent_b {
                edges.push((a, b, *weight));
                if visited.contains_key(&a) {
                    visited.insert(b, a);
                    println!("{}->{}", b, a);
                } else {
                    visited.insert(a, b);
                    println!("{}->{}", a, b);
                }
            }
            println!("===================================================");
        }
        return edges;
    }
}

fn main() {
    let mut graph = Graph::new(vec![
        ('A', 'B', 8),
        ('A', 'G', 11),
        ('A', 'H', 4),
        ('B', 'C', 7),
        ('B', 'E', 4),
        ('B', 'I', 2),
        ('C', 'D', 9),
        ('C', 'E', 14),
        ('D', 'E', 10),
        ('E', 'F', 2),
        ('F', 'G', 1),
        ('F', 'I', 6),
        ('G', 'H', 8),
        ('G', 'I', 7),
    ]);
    let updated_edges = graph.find_mst();
    println!("edges: {:?}", updated_edges)
}
