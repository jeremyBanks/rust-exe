#![allow(unused)]

use crate::*;

#[test]
fn test_cli_entry() {
    expect![[r#"
        Ok(
            CliEntry {
                verbosity: None,
                subcommand: Run(
                    ArgStream {
                        args: [
                            "run",
                            "./target/debug/hello",
                            "world",
                        ],
                        offset: 1,
                    },
                ),
            },
        )
    "#]]
    .assert_debug_eq(&CliEntry::try_new(
        ["./target/debug/hello", "world"]
            .iter()
            .map(Into::into)
            .collect(),
    ));

    expect![[r#"
        Ok(
            CliEntry {
                verbosity: Some(
                    -1,
                ),
                subcommand: Build(
                    ArgStream {
                        args: [
                            "--verbose",
                            "build",
                            "-qq",
                            "./hello.rs",
                            "world",
                        ],
                        offset: 3,
                    },
                ),
            },
        )
    "#]]
    .assert_debug_eq(&CliEntry::try_new(
        ["--verbose", "build", "-qq", "./hello.rs", "world"]
            .iter()
            .map(Into::into)
            .collect(),
    ));
}

#[derive(Debug, Clone)]
pub struct CliEntry {
    pub verbosity: Option<i32>,
    pub subcommand: Subcommand,
}

impl CliEntry {
    pub fn try_new(args: Vec<OsString>) -> eyre::Result<Self> {
        let mut args = ArgStream::new(args);

        dbg!(&args);

        if let Some(leading_path) = args.peek_path() {
            args.push_front("run".into());
        }

        let mut verbosity: Option<i32> = None;

        let options_before_subcommand = args.next_options();

        dbg!(&args);
        let subcommand = args.next_subcommand().expect("missing subcommand");

        let options_after_subcommand = args.next_options();

        let options = options_before_subcommand
            .into_iter()
            .chain(options_after_subcommand);

        for option in options {
            dbg!(&option);
            if let Some(option_bytes) = option.as_bytes().strip_prefix(b"--") {
                match option_bytes {
                    b"verbose" => {
                        verbosity = Some(verbosity.unwrap_or(0) + 1);
                    }
                    b"quiet" => {
                        verbosity = Some(verbosity.unwrap_or(0) - 1);
                    }
                    _ => {
                        eyre::bail!("unrecognized long argument: {:?}", option.to_string_lossy());
                    }
                }
            } else if let Some(option_bytes) = option.as_bytes().strip_prefix(b"-") {
                for arg_byte in option_bytes {
                    match arg_byte {
                        b'v' => {
                            verbosity = Some(verbosity.unwrap_or(0) + 1);
                        }
                        b'q' => {
                            verbosity = Some(verbosity.unwrap_or(0) - 1);
                        }
                        _ => {
                            eyre::bail!(
                                "unrecognized short argument: {:?}",
                                option.to_string_lossy()
                            );
                        }
                    }
                }
            } else {
                unreachable!()
            }
        }

        Ok(CliEntry {
            verbosity,
            subcommand: match subcommand.as_bytes() {
                b"run" => Subcommand::Run(args),
                b"build" => Subcommand::Build(args),
                b"help" => Subcommand::Build(args),
                _ => eyre::bail!(
                    "unrecognized subcommand: {:?}",
                    subcommand.to_string_lossy()
                ),
            },
        })
    }
}

#[derive(Debug, Clone)]
pub struct ArgStream {
    args: Vec<OsString>,
    offset: usize,
}

#[derive(Debug, Clone)]
pub enum Subcommand {
    Run(ArgStream),
    Build(ArgStream),
    Help(ArgStream),
}

impl ArgStream {
    pub fn new(args: Vec<OsString>) -> Self {
        Self { args, offset: 0 }
    }

    pub fn extend(&mut self, args: impl IntoIterator<Item = OsString>) {
        self.args.extend(args);
    }

    pub fn push_back(&mut self, arg: OsString) {
        self.args.push(arg);
    }

    pub fn all(&self) -> &[OsString] {
        &self.args[self.offset..]
    }

    pub fn peek(&self) -> Option<&OsStr> {
        self.args.get(self.offset).map(AsRef::as_ref)
    }

    pub fn push_front(&mut self, arg: OsString) {
        if self.offset > 0 {
            self.offset -= 1;
            self.args[self.offset] = arg;
        } else {
            let old_capacity = self.args.capacity();

            let added_capacity = old_capacity.min(8);
            let new_capacity = old_capacity + added_capacity;

            let new_args = Vec::with_capacity(new_capacity);
            let old_args = std::mem::replace(&mut self.args, new_args);
            self.offset = added_capacity - 2;

            for _ in 0..self.offset {
                self.args.push(OsString::new());
            }

            self.args.push(arg);

            self.args.extend(old_args);
        }
    }

    pub fn pop_front(&mut self) -> Option<OsString> {
        if self.offset >= self.args.len() {
            return None;
        }
        let index = self.offset;
        self.offset += 1;

        let mut next = OsString::new();
        std::mem::swap(&mut self.args[index], &mut next);
        Some(next)
    }

    pub fn pop_back(&mut self) -> Option<OsString> {
        self.args.pop()
    }

    pub fn next(&mut self) -> Option<&OsStr> {
        let index = self.offset;
        self.offset += 1;
        self.args.get(index).map(AsRef::as_ref)
    }

    pub fn next_if<Out>(&mut self, predicate: impl FnOnce(&OsStr) -> Option<Out>) -> Option<Out> {
        let next = self.peek();
        if let Some(next) = next {
            if let Some(next) = predicate(next) {
                self.offset += 1;
                return Some(next);
            }
        }
        None
    }

    pub fn peek_if<Out>(&mut self, predicate: impl FnOnce(&OsStr) -> Option<Out>) -> Option<Out> {
        let next = self.peek();
        if let Some(next) = next {
            if let Some(next) = predicate(next) {
                return Some(next);
            }
        }
        None
    }

    pub fn next_string(&mut self) -> Option<String> {
        self.next_if(|s| Some(s.to_str()?.to_string()))
    }

    pub fn next_parse<Out: FromStr>(&mut self) -> Option<Out> {
        self.next_if(|s| Some(s.to_str()?.parse().ok()?))
    }

    pub fn next_option(&mut self) -> Option<OsString> {
        self.next_if(|s| {
            Some(s)
                .filter(|s| is_argument_like(s.as_bytes()))
                .map(Into::into)
        })
    }

    pub fn next_options(&mut self) -> Vec<OsString> {
        let mut options = Vec::new();
        while let Some(arg) = self.next_option() {
            options.push(arg);
        }
        options
    }

    pub fn next_subcommand(&mut self) -> Option<OsString> {
        self.next_if(|s| {
            Some(s)
                .filter(|s| is_subcommand_like(s.as_bytes()))
                .map(Into::into)
        })
    }

    pub fn next_path(&mut self) -> Option<PathBuf> {
        self.next_if(|s| {
            Some(s)
                .filter(|s| is_path_like(s.as_bytes()))
                .map(Into::into)
        })
    }

    pub fn peek_path(&mut self) -> Option<PathBuf> {
        self.peek_if(|s| {
            Some(s)
                .filter(|s| is_path_like(s.as_bytes()))
                .map(Into::into)
        })
    }

    pub fn len(&self) -> usize {
        self.args.len().saturating_sub(self.offset)
    }
}

impl AsRef<[OsString]> for ArgStream {
    fn as_ref(&self) -> &[OsString] {
        self.all()
    }
}

impl<'arg_stream> IntoIterator for &'arg_stream ArgStream {
    type Item = &'arg_stream OsString;
    type IntoIter = std::slice::Iter<'arg_stream, OsString>;

    fn into_iter(self) -> Self::IntoIter {
        self.all().iter()
    }
}

impl FromIterator<OsString> for ArgStream {
    fn from_iter<T: IntoIterator<Item = OsString>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl FromIterator<String> for ArgStream {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        Self::new(iter.into_iter().map(From::from).collect())
    }
}

impl Iterator for ArgStream {
    type Item = OsString;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop_front()
    }
}

#[test]
fn test_helpers() {
    let hello = b"hello";
    assert_eq!(false, is_argument_like(hello));
    assert_eq!(true, is_subcommand_like(hello));
    assert_eq!(false, is_path_like(hello));

    let hello_world = b"hello world";
    assert_eq!(false, is_argument_like(hello_world));
    assert_eq!(false, is_subcommand_like(hello_world));
    assert_eq!(false, is_path_like(hello_world));

    let hello_slash_world = b"hello/world";
    assert_eq!(false, is_argument_like(hello_slash_world));
    assert_eq!(false, is_subcommand_like(hello_slash_world));
    assert_eq!(true, is_path_like(hello_slash_world));

    let dash_vvvv = b"-vvvv";
    assert_eq!(true, is_argument_like(dash_vvvv));
    assert_eq!(false, is_subcommand_like(dash_vvvv));
    assert_eq!(false, is_path_like(dash_vvvv));

    let dot_slash_dash_vvvv = b"./-vvvv";
    assert_eq!(false, is_argument_like(dot_slash_dash_vvvv));
    assert_eq!(false, is_subcommand_like(dot_slash_dash_vvvv));
    assert_eq!(true, is_path_like(dot_slash_dash_vvvv));
}

fn is_argument_like(s: &[u8]) -> bool {
    s.starts_with(b"-")
}

fn is_path_like(s: &[u8]) -> bool {
    if s.is_empty() || is_argument_like(s) {
        return false;
    }
    for byte in s {
        if matches!(byte, b'/' | b'\\' | b'.') {
            return true;
        }
    }
    false
}

fn is_subcommand_like(s: &[u8]) -> bool {
    if s.is_empty() || is_argument_like(s) {
        return false;
    }
    for byte in s {
        if !matches!(byte, b'a'..=b'z' | b'_' | b'-' | b'0'..=b'9') {
            return false;
        }
    }
    true
}
