//! # Cow (Clone on Write)
//!
//! Cow is a smart pointer that can be used to clone data only when necessary.
//! It is useful when you want to avoid unnecessary cloning of data,
//! especially when the data is large and expensive to clone.
//!
//! Cow can be used in situations where you want to share data between multiple
//! parts of your code without having to worry about ownership and borrowing
//! rules.
//!
//! Cow has 2 variants:
//!
//! - `Borrowed`: This variant holds a reference to the data. It does not own the data and does not clone it.
//! - `Owned`: This variant holds the owned data. It clones the data when it is created and owns it.
//!
//! ## When to use `Cow`?
//!
//! You generally use `Cow` when you want to write a function that:
//! - mainly reads data without modifying it.
//! - may need to modify the data, but only if certain conditions are met
//!   (e.g., if the data contains whitespace that needs to be removed).
//! - when you want to accept both owned and borrowed data as input to a function,
//!   and you want to avoid unnecessary cloning of data.
//!
//! ## When not to use `Cow`?
//! You should avoid using `Cow` when you always need to modify the data,
//! as it will always require cloning, which defeats the purpose of using `Cow`.
//! If you still use `Cow` in such cases, it may lead to unnecessary cloning and
//! performance overhead, as the data will always be cloned regardless of
//! whether it is needed or not.
//!
//! Ref: <https://doc.rust-lang.org/std/borrow/enum.Cow.html>

use std::borrow::Cow;

/// Sanitize String
///
/// The following example demonstrates a function to remove whitespace from a
///  string.
///
/// If the input string contains whitespace, we create an owned `Cow` variant by
/// replacing the whitespace with an empty string. If the input string does not
/// contain whitespace, we can simply return a borrowed `Cow` variant without
/// cloning the data. This approach allows us to avoid unnecessary cloning of
/// the string when it is not needed, while still providing the flexibility to
/// modify the string when necessary.

fn sanitize_string(data: &str) -> Cow<'_, str> {
    if data.contains(' ') {
        // only own the data when the data contains whitespaces.
        Cow::Owned(data.replace(" ", ""))
    } else {
        // If no modification is needed, we can just borrow the data without cloning it.
        Cow::Borrowed(data)
    }
}

fn main() {
    let input_1 = "Hello World";
    let input_2 = "HelloWorld";

    let output_1 = sanitize_string(input_1);
    let output_2 = sanitize_string(input_2);

    // here, `Cow` creates an owned variant and clones the data since it needs
    // modification, hence input 1 and output 1 has different memory location.
    println!("Input 1: '{}', Output 1: '{}'", input_1, output_1);
    println!(
        "address of Input 1: '{:p}', Output 1: '{:p}'",
        input_1.as_ptr(),
        output_1.as_ptr()
    );

    // here, cow borrows immutable reference since it does not contain any
    // whitespace, hence input 2 and output 2 has same memory location
    println!("Input 2: '{}', Output 2: '{}'", input_2, output_2);
    println!(
        "address of Input 2: '{:p}', Output 2: '{:p}'",
        input_2.as_ptr(),
        output_2.as_ptr()
    );
}
