use crate::parse::Ast;
use proc_macro2::Ident;
use syn::{punctuated::Punctuated, token::Comma, Field, Fields};

#[allow(unused)]
pub struct StructModel {
    pub struct_ident: Ident,
    pub named_fields: Punctuated<Field, Comma>,
    pub ast: Ast,
}

// A testable function that analyzes the Ast and
// returns a StructModel.
pub fn analyze(ast: Ast) -> StructModel {
    let struct_ident = ast.ident.clone();

    let named_fields = match &ast.fields {
        Fields::Named(fields) => fields.named.clone(),
        _ => panic!("this derive macro only works on structs with named fields"),
    };

    StructModel {
        struct_ident,
        named_fields,
        ast,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse;
    use quote::quote;

    #[test]
    fn valid_model() {
        let input = quote!(
            pub struct Command {
                executable: String,
                args: Vec<String>,
                env: Vec<String>,
                current_dir: String,
            }
        );

        let item_struct = parse(input);
        let model = analyze(item_struct);
        assert_eq!(&model.struct_ident, &model.ast.ident);
        let mut iter = model.named_fields.iter();

        let mut validate_name = |name: &str| {
            if let Some(id) = iter.next().unwrap().ident.as_ref() {
                assert_eq!(id.to_string(), name.to_string());
            } else {
                panic!(r#"Expecting "{}" but no name"#, name);
            }
        };
        validate_name("executable");
        validate_name("args");
        validate_name("env");
        validate_name("current_dir");
    }
}
