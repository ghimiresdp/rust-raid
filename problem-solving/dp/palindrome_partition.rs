use std::cmp::min;

/// a function to check if the given str is palindrome
fn is_palindrome(word: &str) -> bool {
    let mut from = 0;
    let mut to = word.len() - 1;
    while from < to {
        if word.chars().nth(from).unwrap() != word.chars().nth(to).unwrap() {
            return false;
        }
        from += 1;
        to -= 1;
    }
    return true;
}
/// recursively check if the given string is palindrome
/// as it is a recursive method, it will overlap many sub-problems hence it is
/// not an optimal solution.
fn min_cuts_recursive(word: &str) -> usize {
    if is_palindrome(&word) {
        return 0;
    } else {
        let mut _min = usize::MAX;
        for x in 1..word.len() {
            _min = min(
                _min,
                1 + min_cuts_recursive(&word[..x]) + min_cuts_recursive(&word[x..]),
            );
        }
        return _min;
    };
}

/// # min_cuts
/// dynamic programming approach
/// this approach reuses the overlapped subproblem so that we do not need to
/// find out the palindrome if it is already found.
fn min_cuts(word: &str) -> usize {
    let len = word.len();
    if len == 0 {
        return 0;
    }

    // we create a table to store the palindrome test for all values from range
    // word[0..=0] to word[len-1..=len-1]
    let mut palindrome = vec![vec![false; len]; len];

    // all the characters with length 1 will be palindrome
    // so word[x..=x] is always palindrome.
    for i in 0..len {
        palindrome[i][i] = true;
    }

    // now we need to check if the string slices with length larger than 1 to be
    // palindrome
    // example: DAD might contain DA, AD, DAD
    // example: ADAM might contain AD, DA, AM, ADA, DAM, ADAM

    // start from 2 upto length of the string
    for sub_len in 2..=len {
        // extract the slice from the starting of the substring
        for i in 0..=(len - sub_len) {
            // find the ending position of the substring
            // example: for ADAM with sub_len=2 nd i = 1, substring = A [DA] M
            let j = i + sub_len - 1;

            // given, the smaller substrings being calculated and is palindrome,
            // check if adding characters to the first ant last makes it palindrome
            // again.
            // if starting and ending is same, then the slice is palindrome.
            // (precondition: everything inside [] is palindrome)
            // i.e. palindrome[i + 1][j - 1] == true
            //
            // example: D + [A] + D  = DAD is palindrome
            // example: D + [A] + N = DAN is not palindrome
            // where [A] already is palindrome
            //
            // EXAMPLE: for sub_len == 2, D [] D  = DD, is palindrome
            // EXAMPLE: for sub_len>2, if D [A] D = DAD, is palindrome
            //
            // here palindrome[i + 1][j - 1] = [A] = true (since it is palindrome)
            // for above case  i -> D[A]D <- j, [A] is palindrome
            if word.chars().nth(i).unwrap() == word.chars().nth(j).unwrap()
                && (sub_len == 2 || palindrome[i + 1][j - 1])
            {
                // if above condition passes, the new slice is also palindrome
                // where new slice is [CHAR at i] + [OLD_SLICE] + [CHAR at j]
                palindrome[i][j] = true;
            }
        }
    }

    // now create a vector to store minimum cuts required;
    let mut cuts = vec![usize::MAX; len];
    // for single characters, cuts = 0
    cuts[0] = 0;
    for i in 1..len {
        // if the word is palindrome from first to ith index, then cuts for ith
        // index is 0. i.e. we do not need to cut the palindrome
        if palindrome[0][i] {
            cuts[i] = 0;
        } else {
            // if word [0..i] is not palindrome, then we need to cut once (1+..)
            // and then check for cuts on both left and right side of ith index.
            // i.e. (1 + <left_cuts> + <right_cuts>)
            //
            // for that, we need to check slices from [j..i] to [0..i]
            // if we find palindrome in that range, we update cuts whenever
            // we find minimum cuts that satisfies the precondition
            // where precondition is palindrome[j][i] == true.
            let mut j = i;
            while j >= 1 {
                if palindrome[j][i] {
                    if cuts[j - 1] + 1 < cuts[i] {
                        cuts[i] = cuts[j - 1] + 1;
                    }
                }
                j -= 1;
            }
        }
    }
    return cuts[len - 1];
}

fn main() {
    println!("+ {:-<20} + {:-<20} + {:-<20} + {:-<20} +", "", "", "", "");
    println!(
        "| {:^20} | {:^20} | {:^20} | {:^20} |",
        "word", "is_palindrome", "min_cuts_recursive", "min_cuts_dp"
    );
    println!("+ {:-<20} + {:-<20} + {:-<20} + {:-<20} +", "", "", "", "");
    for word in vec!["dad", "dada", "daad", "abadad", "asadadnan", "abcdefg"] {
        println!(
            "| {:^20} | {:^20} | {:^20} | {:^20} |",
            word,
            is_palindrome(word),
            min_cuts_recursive(word),
            min_cuts(word)
        );
    }
    println!("+ {:-<20} + {:-<20} + {:-<20} + {:-<20} +", "", "", "", "");
    // output
    // + -------------------- + -------------------- + -------------------- + -------------------- +
    // |         word         |    is_palindrome     |  min_cuts_recursive  |     min_cuts_dp      |
    // + -------------------- + -------------------- + -------------------- + -------------------- +
    // |         dad          |         true         |          0           |          0           |
    // |         dada         |        false         |          1           |          1           |
    // |         daad         |         true         |          0           |          0           |
    // |        abadad        |        false         |          1           |          1           |
    // |      asadadnan       |        false         |          2           |          2           |
    // |       abcdefg        |        false         |          6           |          6           |
    // + -------------------- + -------------------- + -------------------- + -------------------- +
}

#[cfg(test)]
mod tests {
    use crate::{is_palindrome, min_cuts, min_cuts_recursive};

    #[test]
    fn test_palindrome() {
        assert!(!is_palindrome("AqwwqB"));
        assert!(is_palindrome("dad"));
    }

    #[test]
    fn test_cuts_recursive() {
        assert_eq!(min_cuts_recursive("AqwwqB"), 2);
        assert_eq!(min_cuts_recursive("daddydda"), 1);
        assert_eq!(min_cuts_recursive("abcde"), 4);
    }

    #[test]
    fn test_cuts() {
        assert_eq!(min_cuts("AqwwqB"), 2);
        assert_eq!(min_cuts("daddydda"), 1);
        assert_eq!(min_cuts("abcde"), 4);
    }
}
