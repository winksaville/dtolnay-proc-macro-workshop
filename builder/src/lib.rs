use proc_macro::TokenStream;

use components::parse;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::Fields;

// This is going to be patterned after the ferrous-systems
// testing-proc-macros blog post:
//   https://ferrous-systems.com/blog/testing-proc-macros/
#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let item_struct = parse(input.into());

    let named_fields = match item_struct.fields {
        Fields::Named(fields) => fields.named,
        _ => panic!("this derive macro only works on structs with named fields"),
    };

    let struct_ident = item_struct.ident;
    let builder_name = format!("{}Builder", struct_ident);
    let builder_ident = Ident::new(&builder_name, Span::call_site());

    let optional_named_fields = named_fields.iter().map(|field| {
        let ident = field.ident.clone();
        let ty = field.ty.clone();
        quote! {
            #ident: Option<#ty>,
        }
    });

    let optional_named_fields_default = named_fields.iter().map(|field| {
        let ident = field.ident.clone();
        quote! {
            #ident: None,
        }
    });

    let rust_ts = quote! {
        #[derive(Debug)] // TODO: Make derive(Debug) conditional on a "feature"?
        pub struct #builder_ident {
            #(#optional_named_fields)*
        }

        impl #struct_ident {
            pub fn builder() -> #builder_ident {
                #builder_ident {
                    #(#optional_named_fields_default)*
                }
            }
        }
    };
    //dbg!(&rust_ts);

    rust_ts.into()
}
