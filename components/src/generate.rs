use crate::analyze::StructModel;
use proc_macro2::{TokenStream, Group, Ident, Literal};
use quote::quote;
use syn::{PathArguments, PathSegment, AngleBracketedGenericArguments};
use proc_macro_error::{abort, abort_call_site};

fn is_optional_field(field: &syn::Field) -> bool {
    let res = match &field.ty {
        syn::Type::Path(ty_path) => {
            let path_segments = &ty_path.path.segments;
            //for (i, seg) in path_segments.iter().enumerate()  {
            //    eprintln!("{} optional_field(): seg={:?}", i, seg);
            //}
            if let Some(first_segment) = path_segments.first() {
                first_segment.ident.to_string().as_str() == "Option"
            } else {
                false
            }
        }
        _ => false,
    };
    //eprintln!("is_optional_field(): res={} ident={:?}", res, &field.ident);
    res
}

fn is_repeated_field(field: &syn::Field) -> bool {
    //eprintln!("is_repeated_field(): ident={:?}", &field.ident);
    let res = match &field.ty {
        syn::Type::Path(ty_path) => {
            let path_segments = &ty_path.path.segments;
            //for (i, seg) in path_segments.iter().enumerate()  {
            //    eprintln!("{} repeated_field(): seg={:?}", i, seg);
            //}
            if let Some(first_segment) = path_segments.first() {
                first_segment.ident.to_string().as_str() == "Vec"
            } else {
                //eprintln!("is_repeated_field(): false, not path_segments");
                false
            }
        }
        _ => {
            //eprintln!("is_repeated_field(): false, expected ty=syn::Type::Path found ty={:?}", &field.ty);
            false
        }
    };
    //eprintln!("is_repeated_field(): res={} ident={:?}", res, &field.ident);
    res
}

fn extract_inner_type(abga: &AngleBracketedGenericArguments) -> Option<&PathSegment> {
    //eprintln!("extract_inner_type_of(): abga={:#?}", abga);
    if let Some(generic_argument) = abga.args.first() {
        //eprintln!("extract_inner_type(): first generic_argument={:?}", generic_argument);
        match generic_argument {
            syn::GenericArgument::Type(syn::Type::Path(type_path)) => {
                if let Some(path_segment) = type_path.path.segments.first() {
                    //eprintln!("extract_inner_type(): first generic_argument path_segment={:?}", path_segment);
                    Some(path_segment)
                } else {
                    None
                }
            }
            _ => None,
        }
    } else {
        None
    }
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
                            extract_inner_type(abga)
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

fn extract_inner_type_of_vector_field(field: &syn::Field) -> Option<&PathSegment> {
    //eprintln!("extract_inner_type_of_vector_field(): field={:#?}", field);
    match &field.ty {
        syn::Type::Path(ty_path) => {
            let path_segments = &ty_path.path.segments;
            //eprintln!("extract_inner_type_of_vector_field(): len={} ty_path.path.segments={:?}", path_segments.len(), path_segments);
            if let Some(first_segment) = path_segments.first() {
                //eprintln!("extract_inner_type_of_vector_field(): first_segment{:?}", first_segment);
                if first_segment.ident.to_string().as_str() == "Vec" {
                    //eprintln!("extract_inner_type_of_vector_field(): first_segment.ident is \"Vec\"");
                    match &first_segment.arguments {
                        PathArguments::AngleBracketed(abga) => {
                            extract_inner_type(abga)
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
                                        abort!(p, "Expecting '=' after {} = found '{}' ", ident.unwrap().to_string(), p.as_char());
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
                        //eprintln!("literal_from_builder_each_attribute: literal={:?}", literal);
                        literal
                    }
                    Err(e) => abort_call_site!("Group Err: {}", e),
                }
            } else {
                abort_call_site!("Expecting one attribute(builder, each = \"xxx\"), there are {} `builder` attributes", segment_count);
            }
        }
        _ => {
            abort_call_site!("Only one attribute(builder, each = \"xxx\") there are {}", field.attrs.len());
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
            //eprintln!( "add_setters: optional_field ident={:?} inner_type={:?}", ident, inner_type);
            quote! {
                fn #ident(&mut self, #ident: #inner_type) -> &mut Self {
                    self.#ident = Some(#ident);
                    self
                }
            }
        } else if is_repeated_field(field) {
            let all_at_a_time = quote! {
                // Generate the "all-at-a-time" method
                fn #ident(&mut self, #ident: #ty) -> &mut Self {
                    self.#ident = Some(#ident);
                    self
                }
            };

            let builder_each_attr_literal = literal_from_builder_each_attribute(field);
            if let Some(each_name) = builder_each_attr_literal {
                //eprintln!("add_setters: each_name={:?}", each_name);
                let each_span = each_name.span();
                let each_name_string: String = each_name.to_string();
                //eprintln!("add_setters: each_name_string.len={} each_name_string={}", each_name_string.len(), each_name_string);
                let each_name_no_quotes: String = each_name_string.chars().filter(|ch| {
                    *ch != '"'
                }).collect();
                //eprintln!("add_setters: each_name_no_quotes.len={} each_name_no_quotes={}", each_name_no_quotes.len(), each_name_no_quotes);
                let each_ident = Some(Ident::new(each_name_no_quotes.as_str(), each_span));
                //eprintln!("add_setters: each_ident={}", each_ident);
                let inner_type = extract_inner_type_of_vector_field(field).unwrap();
                //eprintln!("add_setters: repeated_field HAS builder_each_attr={} ident={:?} inner_type={:#?}", each_ident, ident, inner_type);
                let one_at_a_time = quote! {
                    // Generate the "-at-a-time" method
                    fn #each_ident(&mut self, param: #inner_type) -> &mut Self {
                        if let Some(temp_v) = &mut self.#ident {
                            temp_v.push(param);
                        } else {
                            let mut new_v = Vec::<String>::new();
                            new_v.push(param);
                            self.#ident = Some(new_v);
                        }
                        self
                    }
                };
                if each_ident != ident {
                    //eprintln!( "add_setters: repeated_field HAS builder_each_attr each_ident={:?} != ident={:?} HAS both all & one_at_a_time ty={:?}", each_ident, ident, ty);
                    quote! {
                        #all_at_a_time

                        #one_at_a_time
                    }
                } else {
                    //eprintln!( "add_setters: repeated_field HAS builder_each_attr each_ident={:?} == ident={:?} HAS only one_at_a_time ty={:?}", each_ident, ident, ty);
                    quote! {
                        #one_at_a_time
                    }
                }
            } else {
                //eprintln!( "add_setters: repeated_field NO builder_each_attr HAS only all_at_a_time ident={:?} ty={:#?}", ident, ty);
                quote! {
                    #all_at_a_time
                }
            }

        } else {
            //eprintln!( "add_setters: NON-optional_field ident={:?} ty={:#?}", ident, ty);
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
