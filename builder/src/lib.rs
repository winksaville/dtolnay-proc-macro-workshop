use proc_macro::TokenStream;

use components::{analyze, parse};
use proc_macro2::{Ident, Span};
use quote::quote;

// This is going to be patterned after the ferrous-systems
// testing-proc-macros blog post:
//   https://ferrous-systems.com/blog/testing-proc-macros/
#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse(input.into());

    let struct_model = analyze(ast);

    let struct_ident = &struct_model.struct_ident;
    let builder_name = format!("{}Builder", struct_ident);
    let builder_ident = Ident::new(&builder_name, Span::call_site());

    let optional_named_fields = struct_model.named_fields.iter().map(|field| {
        let ident = field.ident.clone();
        let ty = field.ty.clone();
        quote! {
            #ident: Option<#ty>,
        }
    });

    let optional_named_fields_default = struct_model.named_fields.iter().map(|field| {
        let ident = field.ident.clone();
        quote! {
            #ident: None,
        }
    });

    let add_setters= struct_model.named_fields.iter().map(|field| {
        let ident = field.ident.clone();
        let ty = field.ty.clone();
        quote! {
            fn #ident(&mut self, #ident: #ty) -> &mut Self {
                self.#ident = Some(#ident);
                self
            }
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

        impl #builder_ident {
            #(#add_setters)*
        }
    };
    //dbg!(&rust_ts);

    rust_ts.into()
}
