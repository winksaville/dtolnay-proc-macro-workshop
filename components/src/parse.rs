use proc_macro2::TokenStream;
use syn::{Fields, ItemStruct};

pub type Ast = ItemStruct;

// A testable function that parses the input and
// returns an Ast which isa ItemStruct.
//
// One problem with this fn is that the error it
// catches if the input isn't a `struct` and it
// panics without identifying where the problem is.
pub fn parse(input: TokenStream) -> Ast {
    let parsed_items = syn::parse2::<ItemStruct>(input);
    //eprintln!("parse: parsed_items={:#?}", &parsed_items);
    let item_struct = match parsed_items {
        Ok(item_struct) => {
            match &item_struct.fields {
                Fields::Named(_) => (),
                _ => panic!("this derive macro only works on structs with named fields"),
            };

            item_struct
        }
        Err(e) => {
            // How to show location/span?
            panic!("item is not a struct, e={}", e);
        }
    };

    item_struct
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn valid_syntax() {
        let input = quote!(
            pub struct Command {
                executable: String,
                args: Vec<String>,
                env: Vec<String>,
                current_dir: String,
            }
        );

        let _ast = parse(input);
        //eprintln!("parse::tests::valid_syntax: parse return ast={:?}", ast);
    }

    #[test]
    #[should_panic]
    fn in_valid_syntax() {
        let input = quote!(
            pub fn afn() {}
        );

        parse(input);
        //eprintln!("parse::tests::valid_syntax: parse return ast={:?}", ast);
    }
}
