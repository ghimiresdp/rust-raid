use std::collections::HashMap;

/// # Huffman Coding
/// Suppose we have to transmit a word "SENSELESSNESS" through text message.
/// Each character needs 8 bits of data in ASCII format.
///
/// sending the above text requires 13 X 8 = 104 bits of data.
///
/// Using the Huffman encoding, we can compress the data since it has more
/// repetitions.
///
/// The repetition is as follows:
/// * S: 6,  E: 4,  N: 2,  L: 1
///
/// If we encode the data in a way that the most repeated words are represented by
/// smaller bits, then we can compress the data.
///
/// To solve this problem, we use Huffman Encoding algorithm. This algorithm
/// sorts the data by frequency and then creates a binary tree to assign 0 and 1
/// to each character.
///
/// in this process we add the 2 smallest values to create a new node and repeat
/// until we have 1 node remaining.
///
/// ```
/// * L(1)     N(2)   E(4)  S(6)
/// *   LN(3)  E(4)    S(6)     -> 1 + 2 = 3
/// *     LNE(7)   S(6)       -> 3 + 4 = 7
/// *       SLNE(13)        -> 6 + 7 = 13
/// ```
/// now we build the tree and assign [1] to larger and [0] to smaller values
/// until we reach the end of the tree i.e.13.
///
/// ```
///                 (13)
///                 LNES
///             [1]/    \[0]
///               /      \
///         LNE(7)       S(6)
///      [0]/    \[1]    [0]
///        /      \
///     LN(3)      E(4)
/// [0]/    \[1]   [11]
///   /      \
/// L(1)     N(2)
/// [100]   [101]
/// ```
/// evaluating this, we get the following:
/// * S: 0
/// * E: 11
/// * N: 101
/// * L: 100
///
/// and the word will encode to
/// `0-11-101-0-11-100-11-0-0-101-11-0-0`, which will be just 23 bits long.

#[derive(Debug)]
struct Node {
    symbol: Option<char>,
    freq: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}
impl Node {
    fn new(symbol: char, freq: u32) -> Self {
        Node {
            symbol: Some(symbol),
            freq: freq,
            left: None,
            right: None,
        }
    }
}

struct Huffman {
    codes: HashMap<char, String>,
}

impl Huffman {
    fn new() -> Self {
        Self {
            codes: HashMap::new(),
        }
    }

    fn build_tree(&mut self, nodes: &mut Vec<Box<Node>>) {
        while nodes.len() > 1 {
            nodes.sort_by(|a, b| {
                if a.freq == b.freq {
                    // sort by ascii if freq are same
                    // when different characters have same number of occurrence,
                    // then we might not get predictible results when encoded
                    // at different time, so we have ordered by ascii when we have
                    //same frequency.
                    return a.symbol.cmp(&b.symbol);
                }
                return a.freq.cmp(&b.freq);
            });
            let left = nodes.remove(0); // first node (smallest frequency)
            let right = nodes.remove(0); // after removing the 1st node, 2nd node will be the first.

            // the parent node will add those nodes to the left and right
            // smaller value will be on the left and larger on the right.
            let parent = Box::new(Node {
                freq: left.freq + right.freq,
                symbol: None,
                left: Some(left),
                right: Some(right),
            });
            nodes.push(parent);
        }
    }

    /// In this process, all the nodes from top will start generating the code
    /// The smaller node will be assigned 0 and larger will be 1 recursively
    /// until the node have no children.
    fn generate_codes(&mut self, node: &Box<Node>, code: &str) {
        if let Some(symbol) = node.symbol {
            self.codes.insert(symbol, code.to_string());
        } else {
            self.generate_codes(&node.left.as_ref().unwrap(), &format!("{}{}", code, "0"));
            self.generate_codes(&node.right.as_ref().unwrap(), &format!("{}{}", code, "1"));
        }
    }

    /// Encodes the data string using huffman coding
    /// ## parameters
    /// * `data`(`&str`) - raw data to encode
    ///
    /// ## Returns
    /// A String encoded data.
    fn encode(&mut self, data: &str) -> String {
        let mut hm = HashMap::new();
        for ch in data.chars().into_iter() {
            *hm.entry(ch).or_insert(0) += 1;
        }
        // Create a vector of Box<Node> to store the nodes of the Huffman tree.
        let mut nodes: Vec<Box<Node>> = hm
            .iter()
            .map(|(&symbol, &freq)| Box::new(Node::new(symbol, freq)))
            .collect();

        // Build the Huffman tree using the nodes vector.
        self.build_tree(&mut nodes);

        // Generate the Huffman codes for each character in the tree.
        self.generate_codes(&mut nodes[0], "");

        // Encode the data using the generated Huffman codes.
        let mut encoded = String::new();
        for c in data.chars() {
            encoded.push_str(self.codes.get(&c).unwrap_or(&"".to_string()));
        }
        // Please note that the encoded data is not yet compressed and is just
        // represented in binary format, which needs further processing while
        // transmitting it in the physical layer.
        encoded
    }
}

fn main() {
    let mut hm = Huffman::new();
    println!("{:?}", hm.encode("SENSELESSNESS")); //RESULT: 01110101110011001011100
}

#[cfg(test)]
mod tests {
    use crate::Huffman;

    #[test]
    fn different_frequency() {
        let mut hm = Huffman::new();
        assert_eq!(hm.encode("aaaaaaaaab"), "1111111110".to_owned())
    }
    #[test]
    fn common_frequency() {
        let mut hm = Huffman::new();
        assert_eq!(hm.encode("abbc"), "001101".to_owned())
    }
    #[test]
    fn completely_distinct() {
        let mut hm = Huffman::new();
        assert_eq!(hm.encode("abcde"), "110111000110".to_owned())
    }
    #[test]
    fn senselesssness() {
        let mut hm = Huffman::new();
        assert_eq!(
            hm.encode("SENSELESSNESS"),
            "01110101110011001011100".to_owned()
        )
    }
}
