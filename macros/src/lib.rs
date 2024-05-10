use proc_macro::TokenStream;

/**
 * Examples of procedural macros (proc_macros):
 *
 * Example 1: Basic Procedural macro
 *
 * This macro
 */
#[proc_macro]
pub fn my_proc_macro(input: TokenStream) -> TokenStream {
    input
}
