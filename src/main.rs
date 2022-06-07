use {
    std::time::{SystemTime, UNIX_EPOCH},
    ::{
        eyre::{bail, Result},
        std::{self, ffi::OsString, os::unix::prelude::OsStrExt, path::PathBuf},
    },
};

mod hashing;

pub fn main() -> eyre::Result<()> {
    let mut args = Vec::from_iter(std::env::args_os().skip(1));

    if args.is_empty() {
        args.splice(..0, ["help".into()]);
    } else if is_path_like(args[0].as_bytes()) {
        args.splice(..0, ["run".into()]);
    }

    match args[0].as_bytes() {
        b"help" => help(),
        b"run" => run(PathBuf::from(&args[1]), &args[2..]),
        b"eval" =>
            eval(args[1..].iter().map(|s| s.to_str().unwrap()).collect::<Vec<_>>().join(" "), &[]),
        _ => bail!("unknown command: {:?}", &args[0]),
    }
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
    return false;
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

fn run(path: PathBuf, args: &[OsString]) -> Result<()> {
    let body = std::fs::read_to_string(&path).unwrap();

    compile_and_run(path, body, args)
}

fn eval(body: String, args: &[OsString]) -> Result<()> {
    let body = format!("fn main() {{ println!(\"{{}}\", {{{body}}}); }}");
    let hash = hashing::git_blob_sha1_hex(body.as_bytes());
    let path = std::env::current_dir().unwrap().join(format!("eval_{}.rs", &hash[..8]));

    compile_and_run(path, body, args)
}

fn compile_and_run(path: PathBuf, body: String, args: &[OsString]) -> Result<()> {
    let mtime = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs_f64();

    let hash = hashing::git_blob_sha1_hex(body.as_bytes());

    let filename = path.as_path().file_name().unwrap();

    let identifier =
        ::heck::AsSnakeCase(path.as_path().file_stem().unwrap().to_string_lossy()).to_string();

    let mut manifest = format!(
        r#"
            [package]
            autobins = false
            name = {identifier:?}
            edition = 2021
            version = "0.0.0-mtime-{mtime:.3}"

            [[bins]]
            name = {identifier:?}
            path = {filename:?}

            [dependencies]
        "#
    );

    let file = ::syn::parse_file(&body)?;

    println!("{manifest}");

    std::process::exit(42)
}
