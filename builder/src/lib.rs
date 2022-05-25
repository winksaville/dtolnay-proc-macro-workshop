use components::{analyze, generate, parse};
use proc_macro::TokenStream;

// This is going to be patterned after the ferrous-systems
// testing-proc-macros blog post:
//   https://ferrous-systems.com/blog/testing-proc-macros/
//
// Adding ", attributes(builder)" allows $[builder(xxx = "yyy")]
// attributes on a field. See 07-prepeated-field.rs as
// an example.
#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse(input.into());

    let struct_model = analyze(ast);

    let rust_ts = generate(&struct_model);

    rust_ts.into()
}
