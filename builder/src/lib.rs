use proc_macro::TokenStream;

use parse::parse;

mod parse;

// This is going to be patterned after the ferrous-systems
// testing-proc-macros blog post:
//   https://ferrous-systems.com/blog/testing-proc-macros/
#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    //{
    //    // We may want to use `parse_macro_input!` as it looks
    //    // to have better syntax handling.
    //    use syn::{parse_macro_input, DeriveInput};
    //    let input_ts = input.clone();
    //    let derive_input = parse_macro_input!(input_ts as DeriveInput);
    //    dbg!(&derive_input);
    //}

    parse(input.into());

    TokenStream::new()
}
