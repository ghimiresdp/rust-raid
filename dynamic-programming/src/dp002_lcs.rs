use std::collections::HashMap;

/**
 * -----------------------------------------------------------------------------
 * Longest Common Subsequence problem
 *
 * To execute, please run: cargo run --bin adapter
 * To run tests, please run: cargo test --bin adapter
 * -----------------------------------------------------------------------------
 *
 * Given two strings, find the longest common subsequence (LCS).
 * A subsequence is a sequence that appears in the same relative order, but not
 * necessarily consecutive within another sequence.
 *
 * example:
 * - X = [AGGTAB]
 * - Y = [GXTXAYB]
 *
 * The LCS of X and Y is [GTAB] with length 4
 *
 */
fn main() {
    let a = "AGGTAB";
    let b = "GXTXAYB";
    println!("LCS is : {}", lcs(a, b));
    println!("LCS is : {}", lcs(b, a));
}

/// Computes the length of the Longest Common Subsequence (LCS) between two
/// strings.
/// The LCS of two strings is the longest sequence of characters that appears
/// in the same order (but not necessarily consecutive) in both strings.
/// # Arguments
/// * `a`: The first string.
/// * `b`: The second string.
/// # Returns
/// The length of the LCS of `a` and `b`.
/// Here, we create a matrix [m+1]x[n+1] with values 0 where m is the length of
/// the first word and n is the length of the second word.
///
/// then we traverse through the values using nested loop and add the older value
/// by 1 when new match is found. the data gets updated diagonally until the end
/// the last item i.e. ,data[m][n] will be the length of the longest substring.
///
fn lcs(a: &str, b: &str) -> usize {
    let (m, n) = (a.len(), b.len());
    let mut matrix = vec![vec![0; n + 1]; m + 1]; // Table to store LCS lengths

    // Fill the table
    for i in 1..=m {
        for j in 1..=n {
            if a.chars().nth(i - 1).unwrap() == b.chars().nth(j - 1).unwrap() {
                // one more substring is found so we add 1 to previous diagonal value
                // ⎡ 1   .  ⎤
                // ⎣ .  [v] ⎦
                // the value [v] will be (1 + 1) = 2
                matrix[i][j] = matrix[i - 1][j - 1] + 1;
            } else {
                // no new substring is found so we use max of previously computed values [i, j-1] and [i-1, j]
                // ⎡ .   2  ⎤
                // ⎣ 3  [v] ⎦
                // the value [v] will be max(2, 3) = 3
                matrix[i][j] = std::cmp::max(matrix[i - 1][j], matrix[i][j - 1]);
            }
        }
    }
    // Return the LCS length from the bottom right corner
    matrix[m][n]
}

#[cfg(test)]
mod tests {
    use crate::lcs;

    #[test]
    fn test_0() {
        assert_eq!(lcs("AGGTAB", "GXTXAYB"), 4);
        assert_eq!(lcs("GXTXAYB", "AGGTAB"), 4);
    }
    #[test]
    fn test_1() {
        assert_eq!(lcs("apple", "appple"), 5);
        assert_eq!(lcs("appple", "apple"), 5);
        assert_eq!(lcs("apple", "dxeppla",), 3); // ppl
    }
    #[test]
    fn test_reverse() {
        assert_eq!(lcs("apple", "elppa",), 2); // pp
    }
}
