// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run

use derive_builder::Builder;

#[allow(unused)]
#[derive(Builder)]
pub struct Command {
    executable: Option<String>,
    args: Option<Vec<String>>,
    env: Option<Vec<String>>,
    current_dir: Option<String>,
}

impl Command {
    fn args(&mut self, args: Vec<String>) -> &mut Self {
        self.args = Some(args);
        self
    }
}

fn main() {
    let mut cmd = Command {
        executable: None,
        args: None,
        env: None,
        current_dir: None,
    };

    cmd.args(vec!["build".to_owned(), "--release".to_owned()]);
    assert_eq!(
        cmd.args,
        Some(vec!["build".to_owned(), "--release".to_owned()])
    );

    let mut builder = Command::builder();
    builder.executable("cargo".to_owned());
    assert_eq!(builder.executable, Some("cargo".to_owned()));
    builder.args(vec!["build".to_owned(), "--release".to_owned()]);
    assert_eq!(
        builder.args,
        Some(vec!["build".to_owned(), "--release".to_owned()])
    );
    builder.env(vec![]);
    assert_eq!(builder.env, Some(vec![]));
    builder.current_dir("..".to_owned());
    assert_eq!(builder.current_dir, Some("..".to_owned()));
}
