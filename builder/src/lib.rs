use components::{analyze, generate, parse};
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

// This is going to be patterned after the ferrous-systems
// testing-proc-macros blog post:
//   https://ferrous-systems.com/blog/testing-proc-macros/
//
// Adding ", attributes(builder)" allows $[builder(xxx = "yyy")]
// attributes on a field. See 07-prepeated-field.rs as
// an example.
#[proc_macro_derive(Builder, attributes(builder))]
#[proc_macro_error]
pub fn derive(input: TokenStream) -> TokenStream {
    let res = parse(input.into());
    let ast = match res {
        Ok(item_struct) => item_struct,
        Err(e) => return e.into_compile_error().into(),
    };

    let struct_model = analyze(ast);

    let rust_ts = generate(&struct_model);

    rust_ts.into()
}
