use {
    std::{borrow::BorrowMut, ffi::OsString},
    ::{
        expect_test::{expect, Expect},
        eyre::Result,
        std::{env, fs, path::PathBuf, process::Command},
    },
};

#[test]
fn test_commands() -> Result<()> {
    assert_eq!(Command::new("cargo").args(["build"]).status()?.code(), Some(0));
    let env_path = env::var("PATH").unwrap_or_default();
    let env_dir = env::current_dir()?;
    let env_path = env::join_paths(
        std::iter::once(env_dir.join("target").join("debug")).chain(env::split_paths(&env_path)),
    )?;
    env::set_var("PATH", env_path);

    // usage

    assert_command(Command::new("rust"), expect![[r#"
        status: success
        stdout: 290 bytes/characters
                USAGE:
                  rust [--GLOBAL_OPTIONS ...] [COMMAND] [--COMMAND_OPTIONS...] TARGET [TARGET_ARGS...]
        
                COMMANDS:
                  rust help                    Prints help information
                  rust run TARGET [ARGS ...]   Runs a Rust file
                  rust eval EXPRESSION [...]   Evaluates a Rust expression and prints the result
        stderr: nothing
    "#]])?;

    assert_command(Command::new("rust").arg("help"), expect![[r#"
        status: success
        stdout: 290 bytes/characters
                USAGE:
                  rust [--GLOBAL_OPTIONS ...] [COMMAND] [--COMMAND_OPTIONS...] TARGET [TARGET_ARGS...]
        
                COMMANDS:
                  rust help                    Prints help information
                  rust run TARGET [ARGS ...]   Runs a Rust file
                  rust eval EXPRESSION [...]   Evaluates a Rust expression and prints the result
        stderr: nothing
    "#]])?;

    // eval (no main)

    assert_command(Command::new("rust").arg("eval").args(["2 +", "2", "* 3"]), expect![[r#"
        status: success
        stdout: 2 bytes/characters
                8
        stderr: nothing
    "#]])?;

    // hello world

    assert_command(Command::new("examples/hello.rs"), expect![[r#"
        status: success
        stdout: 12 bytes/characters
                hello, rust
        stderr: nothing
    "#]])?;

    assert_command(Command::new("rust").arg("examples/hello.rs"), expect![[r#"
        status: success
        stdout: 12 bytes/characters
                hello, rust
        stderr: nothing
    "#]])?;

    assert_command(Command::new("rust").args(["run", "examples/hello.rs"]), expect![[r#"
        status: success
        stdout: 12 bytes/characters
                hello, rust
        stderr: nothing
    "#]])?;

    // arguments

    assert_command(Command::new("examples/args.rs").args(["1", "2.0", "three"]), expect![[r#"
        status: success
        stdout: nothing
        stderr: 85 bytes/characters
                [src/main.rs:4] args = [
                    "target/debug/args",
                    "1",
                    "2.0",
                    "three",
                ]

    "#]])?;

    assert_command(
        Command::new("rust").arg("examples/args.rs").args(["1", "2.0", "three"]),
        expect![[r#"
            status: success
            stdout: nothing
            stderr: 85 bytes/characters
                    [src/main.rs:4] args = [
                        "target/debug/args",
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
            stdout: nothing
            stderr: 85 bytes/characters
                    [src/main.rs:4] args = [
                        "target/debug/args",
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
        stderr: nothing
    "#]])?;

    assert_command(Command::new("examples/once_cell.rs"), expect![[r#"
        status: success
        stdout: 12 bytes/characters
                hello, rust
        stderr: nothing
    "#]])?;

    Ok(())
}

fn assert_command(mut command: impl BorrowMut<Command>, expect: Expect) -> Result<()> {
    let output = command.borrow_mut().output()?;

    let status = output
        .status
        .code()
        .map(|code| if code == 0 { "success".to_string() } else { format!("error {code}") })
        .unwrap_or("signal".to_string());

    let stdout = if output.stdout.is_empty() {
        format!("nothing")
    } else {
        let byte_len = output.stdout.len();
        let stdout = String::from_utf8(output.stdout)?;
        let char_len = stdout.chars().count();
        let stdout = stdout.replace("\n", "\n        ");
        let stdout = stdout.trim_end();
        if char_len == byte_len {
            format!("{byte_len} bytes/characters\n        {stdout}")
        } else {
            format!("{byte_len} bytes / {char_len} characters\n        {stdout}")
        }
    };

    let stderr = if output.stderr.is_empty() {
        format!("nothing")
    } else {
        let byte_len = output.stderr.len();
        let stderr = String::from_utf8(output.stderr)?;
        let char_len = stderr.chars().count();
        let stderr = stderr.replace("\n", "\n        ");
        let stderr = stderr.trim_end();
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
