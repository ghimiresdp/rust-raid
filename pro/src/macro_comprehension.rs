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
