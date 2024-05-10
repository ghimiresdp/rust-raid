/// Declarative Macros
///
/// Declarative macros are the macros that are simply input matchers

// Example 1: A basic macro that converts one!() to 1
macro_rules! one {
    () => {
        1
    };
}

/**
 * Example 2: A basic macro that doubles the integer argument passed to it
 */
macro_rules! double {
    ($l:literal) => {
        $l * 2
    };
}

/**
 * Example 3: A macro that mimics List comprehension used in python
 *
 * Note, we can not directly use `for` keyword without brackets in a declarative macro
 * so we need to do something like ((x*2) for x in 0..6)
 */
macro_rules! list {
    (($exp: expr) for $ident: ident in $iter: expr) => {
        $iter.into_iter().map(|$ident| $exp).collect()
    };
    (($exp: expr) for $ident: ident in ($iter: expr) if ($cond: expr)) => {
        $iter
            .into_iter()
            .filter(|$ident| $cond)
            .map(|$ident| $exp)
            .collect()
    };
}

fn main() {
    println!("the value of macro one!() is: {}", one!());
    println!("the double of 5 is: {}", double!(5));

    let compr: Vec<i32> = list!((x*2) for x in 0..6);
    println!("the list comprehension is: {:?}", compr);
    let compr: Vec<i32> = list!((x*2) for x in (0..6 ) if (x%2==0));
    println!("the list comprehension with condition is: {:?}", compr);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_comprehension() {
        let output: Vec<i32> = list!((x*2) for x in 0..6);
        assert_eq!(output, vec![0, 2, 4, 6, 8, 10])
    }

    #[test]
    fn test_comprehension_with_conditions() {
        let output: Vec<i32> = list!((x*3) for x in (0..6) if (x%2==0));
        assert_eq!(output, vec![0, 6, 12])
    }
}
