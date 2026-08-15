use std::{env, os::unix::process::CommandExt, process::Command};

fn main() {
    let launcher = concat!(env!("CARGO_MANIFEST_DIR"), "/tools/aicadia-local");
    let error = Command::new(launcher).args(env::args_os().skip(1)).exec();
    eprintln!("aicadia-dev: could not start the local launcher: {error}");
    std::process::exit(1);
}
