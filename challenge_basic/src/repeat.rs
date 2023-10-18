use std::collections::HashMap;

/**
 * Write a program to find the longest length of a substring without repetition
 * of a character more than 2 times.
 *
 * example: a string "abcdeaabcdef" contains 3 'a's but if you take a substring
 * from b then there will be no characters that gets repeated more than twice.
 * The length of the substring will then be 11.
 */

fn longest_substring_with_less_repeat(data: String) -> usize {
    let mut substring_lengths: Vec<usize> = vec![];
    for start in 0..data.len() {
        let slice = data[start..data.len()].to_owned();
        let mut map: HashMap<u8, usize> = HashMap::new();
        let mut end = start;

        for &ch in slice.as_bytes() {
            end += 1;
            match map.get(&ch) {
                Some(&size) => {
                    if size == 2 {
                        end -= 1;
                        break;
                    } else {
                        map.insert(ch, size + 1);
                    }
                }
                None => {
                    map.insert(ch, 1);
                }
            }
        }

        substring_lengths.push(end - start);
    }
    substring_lengths.iter().max().unwrap().to_owned()
}

fn main() {
    println!(
        "{}",
        longest_substring_with_less_repeat("abcdeaabcdef".to_owned()),   // 11 [bcdeaabcdef]
    );
    println!(
        "{}",
        longest_substring_with_less_repeat("abcdeaabcdaef".to_owned()),   // 9 [bcdeaabcd]
    );
}
