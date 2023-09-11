/**
 * COMPREHENSION (Similar to List Comprehension in python)
 *
 * Rustlang do not have default list comprehension feature, however we are
 * assigned a task to create our own syntax similar to list comprehension.
 * We can not create a comprehension that is 100% similar to python since
 * any expression requires semicolon as a delimiter before we could use another
 * token so we tried to generate a macro that could iterate and map values
 * that has behavior similar to list comprehension.
 *
 * Structure:
 *
 * Pattern 1:
 *  comprehension!{foreach <iterable>; apply <function>}
 * Pattern 2:
 *  comprehension!{foreach <iterable>; apply <function>; where <condition>}
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
