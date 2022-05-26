use crate::analyze::StructModel;
use proc_macro2::{Group, Ident, Literal, TokenStream};
use proc_macro_error::{abort, abort_call_site};
use quote::quote;
use syn::{AngleBracketedGenericArguments, PathArguments, PathSegment};

fn is_name_of_first_path_segment_of_field(field: &syn::Field, name: &str) -> bool {
    let res = match &field.ty {
        syn::Type::Path(ty_path) => {
            let path_segments = &ty_path.path.segments;
            if let Some(first_segment) = path_segments.first() {
                first_segment.ident.to_string().as_str() == name
            } else {
                false
            }
        }
        _ => false,
    };
    res
}
fn is_optional_field(field: &syn::Field) -> bool {
    is_name_of_first_path_segment_of_field(field, "Option")
}

fn is_repeated_field(field: &syn::Field) -> bool {
    is_name_of_first_path_segment_of_field(field, "Vec")
}

fn extract_inner_type(abga: &AngleBracketedGenericArguments) -> &PathSegment {
    if let Some(generic_argument) = abga.args.first() {
        match generic_argument {
            syn::GenericArgument::Type(syn::Type::Path(type_path)) => {
                if let Some(path_segment) = type_path.path.segments.first() {
                    path_segment
                } else {
                    abort!(type_path.path, "No path sgements on argument: {:?}", generic_argument);
                }
            }
            _ => abort!(generic_argument, "Expecting GenericArgument found {:?}", generic_argument),
        }
    } else {
        abort!(abga.args, "No GenericArgument, found {:?}", abga.args);
    }
}

fn extract_inner_type_of_field(field: &syn::Field) -> &PathSegment {
    match &field.ty {
        syn::Type::Path(ty_path) => {
            let path_segments = &ty_path.path.segments;
            if let Some(first_segment) = path_segments.first() {
                match &first_segment.arguments {
                    PathArguments::AngleBracketed(abga) => extract_inner_type(abga),
                    _ => abort!(first_segment, "No segment arguments"),
                }
            } else {
                abort!(path_segments, "No segments");
            }
        }
        _ => abort!(field.ty, "No type"),
    }
}

fn literal_from_builder_each_attribute(field: &syn::Field) -> Option<Literal> {
    match field.attrs.len() {
        0 => None,
        1 => {
            let segment_count = field.attrs[0].path.segments.len();
            if segment_count == 1 {
                match syn::parse2::<Group>(field.attrs[0].tokens.clone()) {
                    Ok(group) => {
                        let mut ident: Option<Ident> = None;
                        let mut literal: Option<Literal> = None;
                        for tt in group.stream().into_iter() {
                            match tt {
                                proc_macro2::TokenTree::Group(g) => {
                                    abort!(g, "Not expecting group `{}` in attribute(builder, each = \"xxx\")", g.to_string());
                                }
                                proc_macro2::TokenTree::Ident(id) => {
                                    if id != "each" {
                                        abort!(id, "expected `builder(each = \"...\")`");
                                    }
                                    ident = Some(id);
                                }
                                proc_macro2::TokenTree::Punct(p) => {
                                    if p.as_char() != '=' {
                                        abort!(
                                            p,
                                            "Expecting '=' after {} = found '{}' ",
                                            ident.unwrap().to_string(),
                                            p.as_char()
                                        );
                                    }
                                }
                                proc_macro2::TokenTree::Literal(l) => {
                                    literal = Some(l.clone());
                                }
                            };
                        }
                        if literal.is_none() {
                            abort!(group, "Expecting a literal, such as `xxx` in attribute(builder, each = \"xxx\"");
                        }
                        literal
                    }
                    Err(e) => abort_call_site!("Group Err: {}", e),
                }
            } else {
                abort_call_site!("Expecting one attribute(builder, each = \"xxx\"), there are {} `builder` attributes", segment_count);
            }
        }
        _ => {
            abort_call_site!(
                "Only one attribute(builder, each = \"xxx\") there are {}",
                field.attrs.len()
            );
        }
    }
}

// A testable function that generates a TokenStream
pub fn generate(struct_model: &StructModel) -> TokenStream {
    let struct_ident = &struct_model.struct_ident;
    let builder_ident = &struct_model.builder_ident;

    let optional_named_fields = struct_model.named_fields.iter().map(|field| {
        let ident = &field.ident;
        let ty = &field.ty;
        if is_optional_field(field) {
            quote! {
                #ident: #ty,
            }
        } else {
            quote! {
                #ident: std::option::Option<#ty>,
            }
        }
    });

    let optional_named_fields_default = struct_model.named_fields.iter().map(|field| {
        let ident = field.ident.clone();
        quote! {
            #ident: std::option::Option::None,
        }
    });

    let add_setters = struct_model.named_fields.iter().map(|field| {
        let ident = field.ident.clone();
        let ty = field.ty.clone();
        if is_optional_field(field) {
            let inner_type = extract_inner_type_of_field(field);
            quote! {
                fn #ident(&mut self, #ident: #inner_type) -> &mut Self {
                    self.#ident = std::option::Option::Some(#ident);
                    self
                }
            }
        } else if is_repeated_field(field) {
            let all_at_a_time = quote! {
                // Generate the "all-at-a-time" method
                fn #ident(&mut self, #ident: #ty) -> &mut Self {
                    self.#ident = std::option::Option::Some(#ident);
                    self
                }
            };

            let builder_each_attr_literal = literal_from_builder_each_attribute(field);
            if let Some(each_name) = builder_each_attr_literal {
                let each_span = each_name.span();
                let each_name_string: String = each_name.to_string();
                let each_name_no_quotes: String =
                    each_name_string.chars().filter(|ch| *ch != '"').collect();
                let each_ident = Some(Ident::new(each_name_no_quotes.as_str(), each_span));
                let inner_type = extract_inner_type_of_field(field);
                let one_at_a_time = quote! {
                    // Generate the "-at-a-time" method
                    fn #each_ident(&mut self, param: #inner_type) -> &mut Self {
                        if let Some(temp_v) = &mut self.#ident {
                            temp_v.push(param);
                        } else {
                            let mut new_v = std::vec::Vec::<String>::new();
                            new_v.push(param);
                            self.#ident = std::option::Option::Some(new_v);
                        }
                        self
                    }
                };
                if each_ident != ident {
                    quote! {
                        #all_at_a_time

                        #one_at_a_time
                    }
                } else {
                    quote! {
                        #one_at_a_time
                    }
                }
            } else {
                quote! {
                    #all_at_a_time
                }
            }
        } else {
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
                struct_ident,
                ident.clone().unwrap()
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

            fn build(&mut self) -> std::result::Result<#struct_ident, std::boxed::Box<dyn std::error::Error>> {
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
