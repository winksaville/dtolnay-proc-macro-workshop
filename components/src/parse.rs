use proc_macro2::TokenStream;
use syn::{Fields, ItemStruct, Result};

pub type Ast = ItemStruct;

// A testable function that parses the input and
// returns an Ast which isa ItemStruct.
//
// One problem with this fn is that the error it
// catches if the input isn't a `struct` and it
// panics without identifying where the problem is.
pub fn parse(input: TokenStream) -> Result<Ast> {
    let parsed_items = syn::parse2::<ItemStruct>(input);
    //eprintln!("parse: parsed_items={:#?}", &parsed_items);

    let item_struct = match parsed_items {
        Ok(item_struct) => {
            match &item_struct.fields {
                Fields::Named(_) => (),
                _ => {
                    let err = syn::Error::new(item_struct.ident.span(), "this derive macro only works on `struct`s with named fields");
                    return Err(err);
                }
            };

            item_struct
        }
        Err(e) => {
            // Happens only if #[derive(Builder)] is NOT on a enum, struct or union and the
            // compiler issues an error before calling our macro:
            //   error[E0774]: `derive` may only be applied to `struct`s, `enum`s and `union`s
            //   --> main.rs:17:1
            //   |
            //   17 | #[derive(Builder, Debug)]
            //   | ^^^^^^^^^^^^^^^^^^^^^^^^^ not applicable here
            //   18 | fn abc() {}
            //   | ----------- not a `struct`, `enum` or `union`
            //
            // Thuse this only happens in testing, so panic is fine.
            panic!("{}", e);
        }
    };

    Ok(item_struct)
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

        let ast = parse(input);
        assert!(&ast.is_ok());
        //eprintln!("parse::tests::valid_syntax: parse return ast={:?}", ast);
    }

    #[test]
    #[should_panic]
    fn invalid_syntax() {
        let input = quote!(
            pub fn afn() {}
        );

        let _ = parse(input);
    }

    #[test]
    fn test_unit_struct() {
        let input = quote!(
            struct UnitStruct;
        );

        let res = parse(input);
        //eprintln!("parse::tests::test_unit_struct: parse res={:?}", res);
        assert!(&res.is_err());
        match &res {
            Err(e) => {
                assert!(e.to_string().contains("only works on `struct`s with named fields"));
            }
            _ => panic!("Should have returned an Err"),
        }
    }
}
