use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    // Add syn crate, the "extra-traits" are needed for Debug traits
    // [see](https://github.com/dtolnay/syn#optional-features)
    //    [dependencies]
    //    syn = { version = "1.0.95", features = ["full", "extra-traits"] }
    let derive_input = parse_macro_input!(input as DeriveInput);

    // Why does printing to stderr always work, even with just:
    //   cargo test
    dbg!(&derive_input);
    //eprintln!("Builder-derive: stderr input={:#?}", &derive_input);

    // Why does printing to stdout only "work" on "rebuilds" even with:
    //   cargo test -- --nocapture --show-output
    //println!("Builder-derive: stdout input={:#?}", &derive_input);

    TokenStream::default()
}
