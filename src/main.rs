#[allow(unused)]
pub(crate) use {
    crate::{hashing::*, run::*},
    ::{
        cargo_lock::Lockfile,
        eyre::Result,
        heck::*,
        quote::quote,
        serde_json::{json, Value as Json},
        std::{
            self,
            env::current_dir,
            ffi::OsString,
            os::unix::prelude::OsStrExt,
            path::PathBuf,
            time::{SystemTime, UNIX_EPOCH},
        },
        syn,
        toml_edit::easy::{toml, Value as Toml},
    },
};

mod hashing;
mod run;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    env_logger::try_init()?;

    let mut args = Vec::from_iter(std::env::args_os().skip(1));

    if args.is_empty() {
        args.splice(..0, ["help".into()]);
    } else if is_path_like(args[0].as_bytes()) {
        args.splice(..0, ["run".into()]);
    }

    match args[0].as_bytes() {
        b"help" => help()?,
        b"run" => run(PathBuf::from(&args[1]), &args[2..])?,
        b"eval" =>
            eval(args[1..].iter().map(|s| s.to_str().unwrap()).collect::<Vec<_>>().join(" "), &[])?,
        _ => {
            eprintln!("no such command: {:?}", &args[0]);
            help()?;
            std::process::exit(1);
        }
    }

    Ok(())
}

fn is_path_like(s: impl AsRef<[u8]>) -> bool {
    let s = s.as_ref();
    if s.starts_with(b"-") {
        return false;
    }
    for byte in s {
        if matches!(byte, b'/' | b'\\' | b'.') {
            return true;
        }
    }
    false
}

fn help() -> Result<()> {
    println!("#!/usr/bin/env rust");

    std::process::exit(0)
}

fn run(path: PathBuf, args: &[OsString]) -> Result<()> {
    let body = std::fs::read_to_string(&path).unwrap();

    compile_and_run(path, body, args)
}

fn eval(body: String, args: &[OsString]) -> Result<()> {
    let body = format!("fn main() {{ println!(\"{{:#?}}\", {{{body}}}); }}");
    let hash = git_blob_sha1_hex(body.as_bytes());
    let path = current_dir().unwrap().join(format!("eval_{}.rs", &hash[..8]));

    compile_and_run(path, body, args)
}
