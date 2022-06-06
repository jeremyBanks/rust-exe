use ::{
    eyre::bail,
    std::{self, ffi::OsString, os::unix::prelude::OsStrExt, path::PathBuf},
};

pub(crate) use crate::hashing::*;

mod hashing;

fn main() -> eyre::Result<()> {
    let args = Vec::from_iter(std::env::args_os().skip(1));

    if args.is_empty() {
        help()
    }

    match args[0].as_bytes() {
        b"help" => help(),
        b"run" => run(PathBuf::from(&args[1]), &args[2..]),
        b"eval" =>
            eval(&args[1..].iter().map(|s| s.to_str().unwrap()).collect::<Vec<_>>().join(" "), &[]),
        _ => bail!("unknown command: {:?}", &args[0]),
    }
}

#[allow(unused)]
fn is_option_like(s: impl AsRef<[u8]>) -> bool {
    s.as_ref().get(0) == Some(&b'-')
}

#[allow(unused)]
fn is_path_like(s: impl AsRef<[u8]>) -> bool {
    for byte in s.as_ref() {
        if matches!(byte, b'/' | b'\\' | b':' | b'.') {
            return true;
        }
    }
    false
}

fn help() -> ! {
    println!("USAGE:");
    println!(
        "  rust [--GLOBAL_OPTIONS ...] [COMMAND] [--COMMAND_OPTIONS...] TARGET [TARGET_ARGS...]"
    );
    println!("");
    println!("COMMANDS:");
    println!("  rust help                    Prints help information");
    println!("  rust run TARGET [ARGS ...]   Runs a Rust file");
    println!("  rust eval EXPRESSION [...]   Evaluates a Rust expression and prints the result");
    println!("");

    std::process::exit(0)
}

fn run(path: PathBuf, args: &[OsString]) -> ! {
    let body = std::fs::read_to_string(path).unwrap();

    eval(&body, args);
}

fn eval(body: &str, args: &[OsString]) -> ! {
    todo!("not é, è, ê and ë");

    std::process::exit(0)
}
