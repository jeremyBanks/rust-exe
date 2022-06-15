use crate::*;

pub fn help() -> Result<()> {
    println!("#!/usr/bin/env rust");

    std::process::exit(0)
}

pub fn run(path: PathBuf, args: &[OsString]) -> Result<()> {
    let body = std::fs::read_to_string(&path).unwrap();

    compile_and_run(path, body, args)
}

pub fn eval(body: String, args: &[OsString]) -> Result<()> {
    let body = format!("fn main() {{ println!(\"{{:#?}}\", {{{body}}}); }}");
    let hash = git_blob_sha1_hex(body.as_bytes());
    let path = current_dir()
        .unwrap()
        .join(format!("eval_{}.rs", &hash[..8]));

    compile_and_run(path, body, args)
}
