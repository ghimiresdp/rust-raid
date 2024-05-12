/**
 * COMPREHENSION (Similar to List Comprehension in python)
 *
 * Rustlang do not have default list comprehension feature, however we are
 * assigned a task to create our own syntax similar to list comprehension.
 *
 * NOTE: the example from `macros/declarative.rs` contains the most compatible
 * version that complies with python's list comprehension.
 *
 * Structure:
 *
 * Pattern 1:
 *  comprehension!{foreach <iterable>; apply <function>}
 * Pattern 2:
 *  comprehension!{foreach <iterable>; apply <function>; where <condition>}
 *
 *
 * To run the code, run the following:
 * =============================================================================
 *
 * cargo run --bin m3
 * cargo test --bin m3
 *
 * =============================================================================
 **/
macro_rules! comprehension {
    (foreach $iterable:expr; apply $function:expr) => {
        $iterable.iter().map($function).collect::<Vec<_>>()
    };
    (foreach $iterable:expr; apply $function:expr; where $condition:expr) => {
        $iterable
            .iter()
            .filter($condition)
            .map($function)
            .collect::<Vec<_>>()
    };
}

// to execute, run:     cargo run --bin comprehension
fn main() {
    let arr = vec![1, 2, 3, 4, 5];
    let out = comprehension!(foreach arr; apply |x|{x * x} );
    println!("Comprehension output: {out:?}");

    let string_len = comprehension!(foreach vec!["apple", "orange", "banana"]; apply |x| {x.len()});
    println!("Comprehension Output [strings]: {string_len:?}");

    let numbers = (0..=10).collect::<Vec<_>>();
    let even_squared = comprehension!(foreach numbers; apply |x|{x*x}; where |&&x|{x%2==0});
    println!("Comprehension Output with filter: {:?}", even_squared);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_squares() {
        let original = (0..5).collect::<Vec<_>>();
        let squares = comprehension!(foreach original; apply |x|{x * x});
        assert_eq!(squares, vec![0, 1, 4, 9, 16])
    }

    #[test]
    fn test_squares_with_filter() {
        let original = (0..5).collect::<Vec<_>>();
        let squares = comprehension!(foreach original; apply |x|{x * x}; where |&&x|{x % 2 == 0});
        assert_eq!(squares, vec![0, 4, 16])
    }
}
