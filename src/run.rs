use crate::*;

pub fn compile_and_run(path: PathBuf, body: String, args: &[OsString]) -> Result<()> {
    let mtime = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs_f64();

    let hash = hashing::git_blob_sha1_hex(body.as_bytes());

    let identifier = path.as_path().file_stem().unwrap().to_string_lossy().to_snake_case();

    let mut manifest = format!(
        r#"
            [package]
            autobins = false
            name = {identifier:?}
            edition = 2021
            version = "0.0.0-mtime-{mtime:.3}"

            [[bins]]
            name = {identifier:?}
            path = "src/main.rs"

            [dependencies]
        "#
    );

    let file = ::syn::parse_file(&body)?;

    println!("{manifest}");

    std::process::exit(42)
}
