use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let _derive_input = parse_macro_input!(input as DeriveInput);
    //dbg!(&derive_input);

    // Create builder identifier for use in `impl Command`
    let builder_ident = Ident::new("builder", Span::call_site());

    // Generate a token stream for `struct CommandBuilder`
    // and `Command::builder()`
    let tokens: proc_macro2::TokenStream = quote! {
        //#[derive(Debug)]
        pub struct CommandBuilder {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }

        impl Command {
            pub fn #builder_ident() -> CommandBuilder {
                CommandBuilder {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }

        impl CommandBuilder {
            fn executable(&mut self, executable: String) -> &mut Self {
                self.executable = Some(executable);
                self
            }
            fn args(&mut self, args: Vec<String>) -> &mut Self {
                self.args = Some(args);
                self
            }
            fn env(&mut self, env: Vec<String>) -> &mut Self {
                self.env = Some(env);
                self
            }
            fn current_dir(&mut self, dir: String) -> &mut Self {
                self.current_dir = Some(dir);
                self
            }
        }
    };

    tokens.into()
}
