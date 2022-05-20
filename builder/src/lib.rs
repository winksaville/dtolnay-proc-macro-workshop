use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

//use utilities::parse::parse;
//mod analyze;

// This is going to be patterned after the ferrous-systems
// testing-proc-macros blog post:
//   https://ferrous-systems.com/blog/testing-proc-macros/
// Although right now everything is here but I'm assuming
// I'll move it to other modules later :)
#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);

    let named_fields = match derive_input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("this derive macro only works on structs with named fields"),
    };

    let struct_ident = derive_input.ident;
    let builder_name = format!("{}Builder", struct_ident);
    let builder_ident = Ident::new(&builder_name, Span::call_site());

    // Got this loop from this blog post; https://blog.turbo.fish/proc-macro-simple-derive/
    // But needed to use `iter()` and `clone()` the fields as I needed two loops
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
