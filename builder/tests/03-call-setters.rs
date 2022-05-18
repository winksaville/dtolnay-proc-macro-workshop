// Generate methods on the builder for setting a value of each of the struct
// fields.
//
//     impl CommandBuilder {
//         fn executable(&mut self, executable: String) -> &mut Self {
//             self.executable = Some(executable);
//             self
//         }
//
//         ...
//     }

use derive_builder::Builder;

#[derive(Builder)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: String,
}

fn main() {
    let mut builder = Command::builder();
    builder.executable("cargo".to_owned());
    assert_eq!(builder.executable, Some("cargo".to_owned()));
    builder.args(vec!["build".to_owned(), "--release".to_owned()]);
    assert_eq!(builder.args, Some(vec!["build".to_owned(), "--release".to_owned()]));
    builder.env(vec![]);
    assert_eq!(builder.env, Some(vec![]));
    builder.current_dir("..".to_owned());
    assert_eq!(builder.current_dir, Some("..".to_owned()));
}
