use proc_macro2::TokenStream;
use quote::quote;
use crate::analyze::StructModel;

// A testable function that generates a TokenStream
pub fn generate(struct_model: &StructModel) -> TokenStream {
    let struct_ident = &struct_model.struct_ident;
    let builder_ident = &struct_model.builder_ident;

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

    rust_ts
}

// TODO: Add tests
#[cfg(test)]
mod tests {

    #[test]
    fn nothing() {
    }

    #[test]
    #[should_panic]
    fn panic() {
        panic!("Panicing");
    }
}
