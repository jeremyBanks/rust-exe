use ::{
    expect_test::{expect, Expect},
    eyre::Result,
    once_cell::sync::{Lazy, OnceCell},
    regex::Regex,
    std::{borrow::BorrowMut, env, process::Command},
};

#[test]
fn test_commands() -> Result<()> {
    // usage

    assert_command(Command::new("rust"), expect![[r#"
        status: success
        stdout: 20 bytes/characters
                #!/usr/bin/env rust
        stderr: none
    "#]])?;

    assert_command(Command::new("rust").arg("help"), expect![[r#"
        status: success
        stdout: 20 bytes/characters
                #!/usr/bin/env rust
        stderr: none
    "#]])?;

    assert_command(Command::new("rust").arg("encrypt"), expect![[r#"
        status: success
        stdout: 20 bytes/characters
                #!/usr/bin/env rust
        stderr: 27 bytes/characters
                no such command: "encrypt"

    "#]])?;

    // eval (no main)

    assert_command(Command::new("rust").arg("eval").args(["2 +", "2", "* 3"]), expect![[r#"
    status: success
    stdout: 2 bytes/characters
            8
    stderr: none
"#]])?;

    assert_command(
        Command::new("rust").arg("eval").args(["Vec::from_iter", "(std::env::args())"]),
        expect![[r#"
        status: success
        stdout: 64 bytes/characters
                [
                    "~/.rust-exe/bin/eval-b5b43243-b4d08cb3",
                ]
        stderr: none
    "#]],
    )?;

    // hello world

    assert_command(Command::new("examples/hello"), expect![[r#"
        status: success
        stdout: 12 bytes/characters
                hello, rust
        stderr: none
    "#]])?;

    assert_command(Command::new("examples/hello.rs"), expect![[r#"
        status: success
        stdout: 12 bytes/characters
                hello, rust
        stderr: none
    "#]])?;

    assert_command(Command::new("rust").arg("examples/hello.rs"), expect![[r#"
        status: success
        stdout: 12 bytes/characters
                hello, rust
        stderr: none
    "#]])?;

    assert_command(Command::new("rust").args(["run", "examples/hello.rs"]), expect![[r#"
        status: success
        stdout: 12 bytes/characters
                hello, rust
        stderr: none
    "#]])?;

    // arguments

    assert_command(Command::new("examples/args.rs").args(["1", "2.0", "three"]), expect![[r#"
        status: success
        stdout: none
        stderr: 177 bytes/characters
                [args.rs:6] working_dir = "/workspaces/rust-exe"
                [args.rs:6] current_exe = "~/.rust-exe/bin/args-f569275b"
                [args.rs:6] args = [
                    "1",
                    "2.0",
                    "three",
                ]

    "#]])?;

    assert_command(
        Command::new("rust").arg("examples/args.rs").args(["1", "2.0", "three"]),
        expect![[r#"
            status: success
            stdout: none
            stderr: 177 bytes/characters
                    [args.rs:6] working_dir = "/workspaces/rust-exe"
                    [args.rs:6] current_exe = "~/.rust-exe/bin/args-f569275b"
                    [args.rs:6] args = [
                        "1",
                        "2.0",
                        "three",
                    ]

        "#]],
    )?;

    assert_command(
        Command::new("rust").args(["run", "examples/args.rs"]).args(["1", "2.0", "three"]),
        expect![[r#"
            status: success
            stdout: none
            stderr: 177 bytes/characters
                    [args.rs:6] working_dir = "/workspaces/rust-exe"
                    [args.rs:6] current_exe = "~/.rust-exe/bin/args-f569275b"
                    [args.rs:6] args = [
                        "1",
                        "2.0",
                        "three",
                    ]

        "#]],
    )?;

    // inferred dependencies

    assert_command(Command::new("examples/EyRe.rs").args(["1", "2.0", "3"]), expect![[r#"
        status: success
        stdout: 16 bytes/characters
                [1.0, 2.0, 3.0]
        stderr: none
    "#]])?;

    assert_command(Command::new("examples/once_cell.rs"), expect![[r#"
        status: success
        stdout: 12 bytes/characters
                hello, rust
        stderr: none
    "#]])?;

    assert_command(Command::new("examples/many.rs"), expect![[r#"
        status: success
        stdout: 62 bytes/characters
                D Ghhd Cdz Ig9u Zs Bzb W Fsb C Bzd G Vw Ig Zvci Bb Yv0gb W Fu
        stderr: none
    "#]])?;

    Ok(())
}

pub fn assert_command(mut command: impl BorrowMut<Command>, expect: Expect) -> Result<()> {
    ensure_rust_bin_in_path();

    let output = command.borrow_mut().output()?;

    let status = output
        .status
        .code()
        .map(|code| if code == 0 { "success".to_string() } else { format!("error {code}") })
        .unwrap_or("signal".to_string());

    let stdout = if output.stdout.is_empty() {
        "none".to_string()
    } else {
        let byte_len = output.stdout.len();
        let stdout = String::from_utf8(output.stdout)?;
        let char_len = stdout.chars().count();
        let stdout = format_output(&stdout);
        if char_len == byte_len {
            format!("{byte_len} bytes/characters\n        {stdout}")
        } else {
            format!("{byte_len} bytes / {char_len} characters\n        {stdout}")
        }
    };

    let stderr = if output.stderr.is_empty() {
        "none".to_string()
    } else {
        let byte_len = output.stderr.len();
        let stderr = String::from_utf8(output.stderr)?;
        let char_len = stderr.chars().count();
        let stderr = format_output(&stderr);
        if char_len == byte_len {
            format!("{byte_len} bytes/characters\n        {stderr}\n")
        } else {
            format!("{byte_len} bytes / {char_len} characters\n        {stderr}\n")
        }
    };

    let s = format!("status: {status}\nstdout: {stdout}\nstderr: {stderr}\n");

    expect.assert_eq(&s);

    Ok(())
}

fn format_output(s: &str) -> String {
    let s = strip_color(s).replace('\n', "\n        ");
    let s = s.trim_end();
    let s = s.replace(::home::home_dir().unwrap().to_str().unwrap(), "~");
    s
}

fn strip_color(s: &str) -> String {
    static ANSI_ESCAPE: Lazy<Regex> = Lazy::new(|| {
        // per https://stackoverflow.com/a/29497680/1114
        Regex::new(r"[\u001b\u009b][\[()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)?[0-9A-ORZcf-nqry=><]")
            .unwrap()
    });
    ANSI_ESCAPE.replace_all(s, "").to_string()
}

fn ensure_rust_bin_in_path() {
    static DONE: OnceCell<()> = OnceCell::new();
    DONE.get_or_init(|| {
        assert_eq!(Command::new("cargo").args(["build"]).status().unwrap().code(), Some(0));

        let mut env_path = env::var("PATH").unwrap_or_default();
        let env_dir = env::current_dir().unwrap();
        let debug_dir = env_dir.join("target").join("debug");
        let examples_dir = env_dir.join("examples");
        if !env_path.contains(&debug_dir.to_str().unwrap()) {
            env_path = env::join_paths(
                [debug_dir, examples_dir].into_iter().chain(env::split_paths(&env_path)),
            )
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
            env::set_var("PATH", env_path);
        }
    });
}
