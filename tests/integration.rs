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

    assert_command(
        Command::new("rust"),
        expect![[r#"
            status: success
            stdout: #!/usr/bin/env rust
            stderr: [src/cli.rs:140] &args = ArgStream {
                        args: [],
                        offset: 0,
                    }
        "#]],
    )?;

    assert_command(
        Command::new("rust").arg("help"),
        expect![[r#"
            status: success
            stdout: #!/usr/bin/env rust
            stderr: [src/cli.rs:140] &args = ArgStream {
                        args: [
                            "help",
                        ],
                        offset: 0,
                    }
        "#]],
    )?;

    assert_command(
        Command::new("rust").arg("-vvvvvvvv"),
        expect![[r#"
            status: success
            stdout: TRACE rust_exe::cli: CliEntry {
                        verbosity: Some(
                            8,
                        ),
                        subcommand: Help(
                            ArgStream {
                                args: [
                                    "-vvvvvvvv",
                                ],
                                offset: 1,
                            },
                        ),
                    }
                    #!/usr/bin/env rust
            stderr: [src/cli.rs:140] &args = ArgStream {
                        args: [
                            "-vvvvvvvv",
                        ],
                        offset: 0,
                    }
                    [src/cli.rs:159] &option = "-vvvvvvvv"
        "#]],
    )?;

    // eval (no main)

    assert_command(
        Command::new("rust")
            .arg("eval")
            .current_dir("/")
            .args(["2 +", "2", "* 3"]),
        expect![[r#"
            status: success
            stdout: 8
            stderr: [src/cli.rs:140] &args = ArgStream {
                        args: [
                            "eval",
                            "2 +",
                            "2",
                            "* 3",
                        ],
                        offset: 0,
                    }
        "#]],
    )?;

    assert_command(
        Command::new("rust")
            .arg("eval")
            .current_dir("/")
            .args(["Vec::from_iter", "(std::env::args())"]),
        expect![[r#"
            status: success
            stdout: [
                        "~/.rust-exe/bin/eval-b5b43243-52c39236",
                    ]
            stderr: [src/cli.rs:140] &args = ArgStream {
                        args: [
                            "eval",
                            "Vec::from_iter",
                            "(std::env::args())",
                        ],
                        offset: 0,
                    }
        "#]],
    )?;

    // hello world

    assert_command(
        Command::new("examples/hello"),
        expect![[r#"
            status: error 101
            stdout: none
            stderr: [src/cli.rs:140] &args = ArgStream {
                        args: [
                            "examples/hello",
                        ],
                        offset: 0,
                    }
                    The application panicked (crashed).
                    Message:  attempt to subtract with overflow
                    Location: src/arg_stream.rs:43
        
                    Backtrace omitted. Run with RUST_BACKTRACE=1 environment variable to display it.
                    Run with RUST_BACKTRACE=full to include source snippets.
        "#]],
    )?;

    assert_command(
        Command::new("examples/hello.rs"),
        expect![[r#"
            status: error 101
            stdout: none
            stderr: [src/cli.rs:140] &args = ArgStream {
                        args: [
                            "examples/hello.rs",
                        ],
                        offset: 0,
                    }
                    The application panicked (crashed).
                    Message:  attempt to subtract with overflow
                    Location: src/arg_stream.rs:43
        
                    Backtrace omitted. Run with RUST_BACKTRACE=1 environment variable to display it.
                    Run with RUST_BACKTRACE=full to include source snippets.
        "#]],
    )?;

    assert_command(
        Command::new("rust").arg("examples/hello.rs"),
        expect![[r#"
            status: error 101
            stdout: none
            stderr: [src/cli.rs:140] &args = ArgStream {
                        args: [
                            "examples/hello.rs",
                        ],
                        offset: 0,
                    }
                    The application panicked (crashed).
                    Message:  attempt to subtract with overflow
                    Location: src/arg_stream.rs:43
        
                    Backtrace omitted. Run with RUST_BACKTRACE=1 environment variable to display it.
                    Run with RUST_BACKTRACE=full to include source snippets.
        "#]],
    )?;

    assert_command(
        Command::new("rust").args(["run", "examples/hello.rs"]),
        expect![[r#"
            status: success
            stdout: hello, rust
            stderr: [src/cli.rs:140] &args = ArgStream {
                        args: [
                            "run",
                            "examples/hello.rs",
                        ],
                        offset: 0,
                    }
        "#]],
    )?;

    // arguments

    assert_command(
        Command::new("examples/args.rs").args(["1", "2.0", "three"]),
        expect![[r#"
            status: success
            stdout: none
            stderr: [src/cli.rs:140] &args = ArgStream {
                        args: [
                            "examples/args.rs",
                            "1",
                            "2.0",
                            "three",
                        ],
                        offset: 0,
                    }
                    [args.rs:6] working_dir = "."
                    [args.rs:6] current_exe = "~/.rust-exe/bin/args-f569275b"
                    [args.rs:6] args = [
                        "1",
                        "2.0",
                        "three",
                    ]
        "#]],
    )?;

    assert_command(
        Command::new("rust")
            .arg("examples/args.rs")
            .args(["1", "2.0", "three"]),
        expect![[r#"
            status: success
            stdout: none
            stderr: [src/cli.rs:140] &args = ArgStream {
                        args: [
                            "examples/args.rs",
                            "1",
                            "2.0",
                            "three",
                        ],
                        offset: 0,
                    }
                    [args.rs:6] working_dir = "."
                    [args.rs:6] current_exe = "~/.rust-exe/bin/args-f569275b"
                    [args.rs:6] args = [
                        "1",
                        "2.0",
                        "three",
                    ]
        "#]],
    )?;

    assert_command(
        Command::new("rust")
            .args(["run", "examples/args.rs"])
            .args(["1", "2.0", "three"]),
        expect![[r#"
            status: success
            stdout: none
            stderr: [src/cli.rs:140] &args = ArgStream {
                        args: [
                            "run",
                            "examples/args.rs",
                            "1",
                            "2.0",
                            "three",
                        ],
                        offset: 0,
                    }
                    [args.rs:6] working_dir = "."
                    [args.rs:6] current_exe = "~/.rust-exe/bin/args-f569275b"
                    [args.rs:6] args = [
                        "1",
                        "2.0",
                        "three",
                    ]
        "#]],
    )?;

    // inferred dependencies

    assert_command(
        Command::new("examples/EyRe.rs").args(["1", "2.0", "3"]),
        expect![[r#"
            status: success
            stdout: [1.0, 2.0, 3.0]
            stderr: [src/cli.rs:140] &args = ArgStream {
                        args: [
                            "examples/EyRe.rs",
                            "1",
                            "2.0",
                            "3",
                        ],
                        offset: 0,
                    }
        "#]],
    )?;

    assert_command(
        Command::new("examples/once_cell.rs"),
        expect![[r#"
            status: error 101
            stdout: none
            stderr: [src/cli.rs:140] &args = ArgStream {
                        args: [
                            "examples/once_cell.rs",
                        ],
                        offset: 0,
                    }
                    The application panicked (crashed).
                    Message:  attempt to subtract with overflow
                    Location: src/arg_stream.rs:43
        
                    Backtrace omitted. Run with RUST_BACKTRACE=1 environment variable to display it.
                    Run with RUST_BACKTRACE=full to include source snippets.
        "#]],
    )?;

    assert_command(
        Command::new("examples/many.rs"),
        expect![[r#"
            status: error 101
            stdout: none
            stderr: [src/cli.rs:140] &args = ArgStream {
                        args: [
                            "examples/many.rs",
                        ],
                        offset: 0,
                    }
                    The application panicked (crashed).
                    Message:  attempt to subtract with overflow
                    Location: src/arg_stream.rs:43
        
                    Backtrace omitted. Run with RUST_BACKTRACE=1 environment variable to display it.
                    Run with RUST_BACKTRACE=full to include source snippets.
        "#]],
    )?;

    Ok(())
}

pub fn assert_command(mut command: impl BorrowMut<Command>, expect: Expect) -> Result<()> {
    ensure_rust_bin_in_path();

    let output = command.borrow_mut().output()?;

    let status = output
        .status
        .code()
        .map(|code| {
            if code == 0 {
                "success".to_string()
            } else {
                format!("error {code}")
            }
        })
        .unwrap_or_else(|| "signal".to_string());

    let stdout = if output.stdout.is_empty() {
        "none".to_string()
    } else {
        format_output(&String::from_utf8(output.stdout)?)
    };

    let stderr = if output.stderr.is_empty() {
        "none".to_string()
    } else {
        format_output(&String::from_utf8(output.stderr)?)
    };

    let s = format!("status: {status}\nstdout: {stdout}\nstderr: {stderr}\n");

    expect.assert_eq(&s);

    Ok(())
}

fn format_output(s: &str) -> String {
    let s = strip_color(s).replace('\n', "\n        ");
    let s = s.trim_end();
    let s = s.replace(env::current_dir().unwrap().to_str().unwrap(), ".");
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
        assert_eq!(
            Command::new("cargo")
                .args(["build"])
                .status()
                .unwrap()
                .code(),
            Some(0)
        );

        let mut env_path = env::var("PATH").unwrap_or_default();
        let env_dir = env::current_dir().unwrap();
        let debug_dir = env_dir.join("target").join("debug");
        let examples_dir = env_dir.join("examples");
        if !env_path.contains(&debug_dir.to_str().unwrap()) {
            env_path = env::join_paths(
                [debug_dir, examples_dir]
                    .into_iter()
                    .chain(env::split_paths(&env_path)),
            )
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
            env::set_var("PATH", env_path);
        }
    });
}
