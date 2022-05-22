use components::{analyze, generate, parse};
use proc_macro::TokenStream;

// This is going to be patterned after the ferrous-systems
// testing-proc-macros blog post:
//   https://ferrous-systems.com/blog/testing-proc-macros/
#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse(input.into());

    let struct_model = analyze(ast);

    let rust_ts = generate(&struct_model);

    rust_ts.into()
}
