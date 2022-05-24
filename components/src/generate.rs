use crate::analyze::StructModel;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{PathArguments, PathSegment};

// A testable function that generates a TokenStream
pub fn generate(struct_model: &StructModel) -> TokenStream {
    let struct_ident = &struct_model.struct_ident;
    let builder_ident = &struct_model.builder_ident;

    fn is_optional_field(field: &syn::Field) -> bool {
        let res = match &field.ty {
            syn::Type::Path(ty_path) => {
                let path_segments = &ty_path.path.segments;
                //for (i, seg) in path_segments.iter().enumerate()  {
                //    eprintln!("{} optional_field(): seg={:?}", i, seg);
                //}
                if let Some(first_segment) = path_segments.first() {
                    if first_segment.ident.to_string().as_str() == "Option" {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            _ => false,
        };
        //eprintln!("is_optional_field(): res={} ident={:?}", res, &field.ident);
        res
    }

    fn extract_inner_type_of_optional_field(field: &syn::Field) -> Option<&PathSegment> {
        //eprintln!("extract_inner_type_of_optional_field(): field={:#?}", field);
        match &field.ty {
            syn::Type::Path(ty_path) => {
                let path_segments = &ty_path.path.segments;
                //eprintln!("extract_inner_type_of_optional_field(): len={} ty_path.path.segments={:?}", path_segments.len(), path_segments);
                if let Some(first_segment) = path_segments.first() {
                    //eprintln!("extract_inner_type_of_optional_field(): first_segment{:?}", first_segment);
                    if first_segment.ident.to_string().as_str() == "Option" {
                        //eprintln!("extract_inner_type_of_optional_field(): first_segment.ident is \"Option\"");
                        match &first_segment.arguments {
                            PathArguments::AngleBracketed(abga) => {
                                //eprintln!("extract_inner_type_of_optional_field(): abga.args.len={} abga.args={:?}", abga.args.len(), abga.args);
                                if let Some(generic_argument) = abga.args.first() {
                                    //eprintln!("extract_inner_type_of_optional_field(): first generic_argument={:?}", generic_argument);
                                    match generic_argument {
                                        syn::GenericArgument::Type(ty) => {
                                            //eprintln!("extract_inner_type_of_optional_field(): first generic_argument ty={:?}", ty);
                                            match ty {
                                                syn::Type::Path(type_path) => {
                                                    if let Some(path_segment) =
                                                        type_path.path.segments.first()
                                                    {
                                                        //eprintln!("extract_inner_type_of_optional_field(): first generic_argument path_segment={:?}", path_segment);
                                                        Some(path_segment)
                                                    } else {
                                                        None
                                                    }
                                                }
                                                _ => None,
                                            }
                                        }
                                        _ => None,
                                    }
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    let optional_named_fields = struct_model.named_fields.iter().map(|field| {
        let ident = &field.ident;
        let ty = &field.ty;
        if is_optional_field(field) {
            quote! {
                #ident: #ty,
            }
        } else {
            quote! {
                #ident: Option<#ty>,
            }
        }
    });

    let optional_named_fields_default = struct_model.named_fields.iter().map(|field| {
        let ident = field.ident.clone();
        quote! {
            #ident: None,
        }
    });

    let add_setters = struct_model.named_fields.iter().map(|field| {
        let ident = field.ident.clone();
        let ty = field.ty.clone();
        if is_optional_field(field) {
            let inner_type = extract_inner_type_of_optional_field(field).unwrap();
            //eprintln!(
            //    "add_setters: optional_field ident={:?} inner_type={:?}",
            //    ident, inner_type
            //);
            quote! {
                fn #ident(&mut self, #ident: #inner_type) -> &mut Self {
                    self.#ident = Some(#ident);
                    //self.#ident = None
                    self
                }
            }
        } else {
            //eprintln!(
            //    "add_setters: NON-optional_field ident={:?} ty={:?}",
            //    ident, ty
            //);
            quote! {
                fn #ident(&mut self, #ident: #ty) -> &mut Self {
                    self.#ident = Some(#ident);
                    self
                }
            }
        }
    });

    let add_assignments = struct_model.named_fields.iter().map(|field| {
        let ident = field.ident.clone();
        if is_optional_field(field) {
            quote! {
                let #ident = self.#ident.take();
            }
        } else {
            let error_string = format!(
                "{} field: `{}` not set",
                struct_ident.to_string(),
                ident.clone().unwrap().to_string()
            );

            quote! {
                let #ident = if let Some(v) = self.#ident.take() {
                    v
                } else {
                    return Err(#error_string.into());
                };
            }
        }
    });

    let named_fields = struct_model.named_fields.iter().map(|field| {
        let ident = field.ident.clone();
        quote! {
            #ident
        }
    });

    let rust_ts = quote! {
        #[derive(Debug)]    // Make conditional on feature?
        #[allow(unused)]    // If we derive(Debug) I need to mark as allow(unused), otherwise we get this warning
                            // if a field, such as current_dir, is_optional_field:
                            //  warning: field is never read: `current_dir`
                            //    --> main.rs:22:5
                            //     |
                            //  22 |     current_dir: Option<String>,
                            //     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
                            //     |
                            //     = note: `#[warn(dead_code)]` on by default
                            //  note: `CommandBuilder` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
                            //    --> main.rs:17:10
                            //     |
                            //  17 | #[derive(Builder, Debug)]
                            //     |          ^^^^^^^
                            //     = note: this warning originates in the derive macro `Debug` (in Nightly builds, run with -Z macro-backtrace for more info)
                            //
                            //  warning: `proc-macro-workshop` (bin "workshop") generated 1 warning
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

            fn build(&mut self) -> Result<#struct_ident, Box<dyn std::error::Error>> {
                #(#add_assignments)*

                Ok(#struct_ident {
                    #(#named_fields),*
                })
            }
        }
    };
    //dbg!(&rust_ts);

    rust_ts
}

// TODO: Add tests
#[cfg(test)]
mod tests {

    #[test]
    fn nothing() {}

    #[test]
    #[should_panic]
    fn panic() {
        panic!("Panicing");
    }
}
