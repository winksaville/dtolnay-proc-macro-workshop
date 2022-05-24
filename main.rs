// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run

//use proc_macro2::{Span, Ident, TokenStream};
//use quote::{quote, __private::ext::RepToTokensExt};
//use utilities::parse::parse;
//use syn::{ItemStruct, FieldsNamed, Fields, token::Token};

use derive_builder::Builder;

#[allow(unused)]
#[derive(Builder, Debug)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: Option<String>,
    //current_dir: String,
}
//
//// This is what "my-solutions" ended up atk
//pub struct CommandBuilder {
//    executable: Option<String>,
//    args: Option<Vec<String>>,
//    env: Option<Vec<String>>,
//    current_dir: Option<String>,
//}
//
//impl Command {
//    pub fn builder() -> CommandBuilder {
//        CommandBuilder {
//            executable: None,
//            args: None,
//            env: None,
//            current_dir: None,
//        }
//    }
//}
//
//#[allow(unused)]
//impl CommandBuilder {
//    fn executable(&mut self, executable: String) -> &mut Self {
//        self.executable = Some(executable);
//        self
//    }
//    fn args(&mut self, args: Vec<String>) -> &mut Self {
//        self.args = Some(args);
//        self
//    }
//    fn env(&mut self, env: Vec<String>) -> &mut Self {
//        self.env = Some(env);
//        self
//    }
//    fn current_dir(&mut self, dir: String) -> &mut Self {
//        self.current_dir = Some(dir);
//        self
//    }
//
//    fn build(&mut self) -> Result<Command, Box<dyn std::error::Error>> {
//        let executable = if let Some(v) = self.executable.take() {
//            v
//        } else {
//            return Err("executable not set".into());
//        };
//        let args = if let Some(v) = self.args.take() {
//            v
//        } else {
//            return Err("args not set".into());
//        };
//        let env = if let Some(v) = self.env.take() {
//            v
//        } else {
//            return Err("env not set".into());
//        };
//        let current_dir = if let Some(v) = self.current_dir.take() {
//            v
//        } else {
//            return Err("current_dir not set".into());
//        };
//
//        Ok(Command {
//            executable,
//            args,
//            env,
//            current_dir,
//        })
//    }
//}
//
//fn builder_test() -> Result<(), Box<dyn std::error::Error>> {
//    let mut builder = Command::builder();
//
//    let expected_executable = "cargo".to_owned();
//    let expected_args = vec!["build".to_owned(), "--release".to_owned()];
//    let expected_env = Vec::<String>::new();
//    let expected_current_dir = "..".to_owned();
//
//    builder.executable(expected_executable.clone());
//    assert_eq!(builder.executable, Some(expected_executable.clone()));
//    builder.args(expected_args.clone());
//    assert_eq!(builder.args, Some(expected_args.clone()));
//    builder.env(expected_env.clone());
//    assert_eq!(builder.env, Some(expected_env.clone()));
//    builder.current_dir(expected_current_dir.clone());
//    assert_eq!(builder.current_dir, Some(expected_current_dir.clone()));
//
//    let cmd = builder.build()?;
//    assert_eq!(cmd.executable, expected_executable);
//    assert_eq!(cmd.args, expected_args);
//    assert_eq!(cmd.env, expected_env);
//    assert_eq!(cmd.current_dir, expected_current_dir);
//
//    Ok(())
//}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = Command::builder();
    eprintln!("builder={:#?}", builder);

    let expected_executable = "cargo".to_owned();
    let expected_args = vec!["build".to_owned(), "--release".to_owned()];
    let expected_env = Vec::<String>::new();
    let expected_current_dir = "..".to_owned();

    builder.executable(expected_executable.clone());
    assert_eq!(builder.executable, Some(expected_executable.clone()));
    builder.args(expected_args.clone());
    assert_eq!(builder.args, Some(expected_args.clone()));
    builder.env(expected_env.clone());
    assert_eq!(builder.env, Some(expected_env.clone()));
    builder.current_dir(expected_current_dir.clone());
    assert_eq!(builder.current_dir, Some(expected_current_dir.clone()));

    let cmd = builder.build()?;
    eprintln!("cmd={:#?}", cmd);
    assert_eq!(cmd.executable, expected_executable);
    assert_eq!(cmd.args, expected_args);
    assert_eq!(cmd.env, expected_env);
    assert_eq!(cmd.current_dir, Some(expected_current_dir));
    Ok(())
}
