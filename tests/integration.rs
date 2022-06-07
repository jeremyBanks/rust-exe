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
        env::split_paths(&env_path).chain(Some(env_dir.join("target").join("debug"))),
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
        status: error 42
        stdout: 297 bytes/characters
        
                            [package]
                            autobins = false
                            name = "eval_6722ac8e"
                            edition = 2021
                            version = "0.0.0-mtime-1654563716.775"
        
                            [[bins]]
                            name = "eval_6722ac8e"
                            path = "eval_6722ac8e.rs"
        
                            [dependencies]
        stderr: nothing
    "#]])?;

    // hello world

    assert_command(Command::new("examples/hello.rs"), expect![[r#"
        status: error 42
        stdout: 273 bytes/characters
        
                            [package]
                            autobins = false
                            name = "hello"
                            edition = 2021
                            version = "0.0.0-mtime-1654563716.779"
        
                            [[bins]]
                            name = "hello"
                            path = "hello.rs"
        
                            [dependencies]
        stderr: nothing
    "#]])?;

    assert_command(Command::new("rust").arg("examples/hello.rs"), expect![[r#"
        status: error 42
        stdout: 273 bytes/characters
        
                            [package]
                            autobins = false
                            name = "hello"
                            edition = 2021
                            version = "0.0.0-mtime-1654563716.781"
        
                            [[bins]]
                            name = "hello"
                            path = "hello.rs"
        
                            [dependencies]
        stderr: nothing
    "#]])?;

    assert_command(Command::new("rust").args(["run", "examples/hello.rs"]), expect![[r#"
        status: error 42
        stdout: 273 bytes/characters
        
                            [package]
                            autobins = false
                            name = "hello"
                            edition = 2021
                            version = "0.0.0-mtime-1654563716.784"
        
                            [[bins]]
                            name = "hello"
                            path = "hello.rs"
        
                            [dependencies]
        stderr: nothing
    "#]])?;

    // arguments

    assert_command(Command::new("examples/args.rs").args(["1", "2.0", "three"]), expect![[r#"
        status: error 42
        stdout: 270 bytes/characters
        
                            [package]
                            autobins = false
                            name = "args"
                            edition = 2021
                            version = "0.0.0-mtime-1654563716.787"
        
                            [[bins]]
                            name = "args"
                            path = "args.rs"
        
                            [dependencies]
        stderr: nothing
    "#]])?;

    assert_command(
        Command::new("rust").arg("examples/args.rs").args(["1", "2.0", "three"]),
        expect![[r#"
            status: error 42
            stdout: 270 bytes/characters
        
                                [package]
                                autobins = false
                                name = "args"
                                edition = 2021
                                version = "0.0.0-mtime-1654563716.789"
        
                                [[bins]]
                                name = "args"
                                path = "args.rs"
        
                                [dependencies]
            stderr: nothing
        "#]],
    )?;

    assert_command(
        Command::new("rust").args(["run", "examples/args.rs"]).args(["1", "2.0", "three"]),
        expect![[r#"
            status: error 42
            stdout: 270 bytes/characters
        
                                [package]
                                autobins = false
                                name = "args"
                                edition = 2021
                                version = "0.0.0-mtime-1654563716.792"
        
                                [[bins]]
                                name = "args"
                                path = "args.rs"
        
                                [dependencies]
            stderr: nothing
        "#]],
    )?;

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
