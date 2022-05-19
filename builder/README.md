# Builder proc-macro

My first partial solution, up to `05-method-chaining`, is on branch
[my-solution]() and it just uses `quote!` to generate code for
`Command` and was not generalized at all. It was informative as it was
my first attempt at creating a `Builder`.

This solution is intended to be "general" and create the `Buiilder` pattern
for "most" structs. It is modeled after the ferrous-systems
[testing-proc-macros](https://ferrous-systems.com/blog/testing-proc-macros/).
I have no idea if this is a good approach or not but I'm going to
give it a try.

# Building, running, testing

Currently, in `/main.rs`, I have my "first" solution is hard coded and something
like this is what this solution should eventaully generate. Change directory
to the root of the repo and run `cargo run`:
```
wink@3900x 22-05-19T21:29:41.984Z:~/prgs/rust/forks/dtolnay-proc-macro-workshop (my-solutions-2)
$ cargo run
   Compiling proc-macro-workshop v0.0.0 (/home/wink/prgs/rust/forks/dtolnay-proc-macro-workshop)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/workshop`
[main.rs:123] cmd = Command {
    executable: "ex",
    args: [],
    env: [],
    current_dir: "cur_dir",
}
wink@3900x 22-05-19T21:29:44.052Z:~/prgs/rust/forks/dtolnay-proc-macro-workshop (my-solutions-2)
```

Next `cd builder` and run `cargo test` which passes `01-parse.rs`:
```
wink@3900x 22-05-19T21:30:57.161Z:~/prgs/rust/forks/dtolnay-proc-macro-workshop/builder (my-solutions-2)
$ cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.01s
     Running unittests (/home/wink/prgs/rust/forks/dtolnay-proc-macro-workshop/target/debug/deps/derive_builder-264b26286a508108)

running 2 tests
test parse::tests::in_valid_syntax - should panic ... ok
test parse::tests::valid_syntax ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/progress.rs (/home/wink/prgs/rust/forks/dtolnay-proc-macro-workshop/target/debug/deps/tests-49d9e9e8fe044e96)

running 1 test
   Compiling derive_builder-tests v0.0.0 (/home/wink/prgs/rust/forks/dtolnay-proc-macro-workshop/target/tests/derive_builder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s


test tests/01-parse.rs ... ok


test tests ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.19s

   Doc-tests derive_builder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

wink@3900x 22-05-19T21:31:01.649Z:~/prgs/rust/forks/dtolnay-proc-macro-workshop/builder (my-solutions-2)
```

