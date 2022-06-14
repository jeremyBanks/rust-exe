#![allow(unused)]
use crate::*;

#[derive(Debug, Clone)]
pub struct ArgStream {
    args: Vec<OsString>,
    offset: usize,
}

impl ArgStream {
    pub fn new(args: Vec<OsString>) -> Self {
        Self { args, offset: 0 }
    }

    pub fn all(&self) -> &[OsString] {
        &self.args[self.offset..]
    }

    pub fn peek(&self) -> Option<&OsStr> {
        self.args.get(self.offset).map(AsRef::as_ref)
    }

    pub fn pop(&mut self) -> Option<OsString> {
        if self.offset >= self.args.len() {
            return None;
        }
        let index = self.offset;
        self.offset += 1;

        let mut next = OsString::new();
        std::mem::swap(&mut self.args[index], &mut next);
        Some(next)
    }

    pub fn next(&mut self) -> Option<&OsStr> {
        let index = self.offset;
        self.offset += 1;
        self.args.get(index).map(AsRef::as_ref)
    }

    pub fn extend(&mut self, args: impl IntoIterator<Item = OsString>) {
        self.args.extend(args);
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
        self.pop()
    }
}

#[test]
fn test_arg_stream() {}

#[test]
fn test_helpers() {
    let hello = b"hello";
    assert_eq!(false, is_argument_like(hello));
    assert_eq!(true, is_subcommand_like(hello));
    assert_eq!(false, is_path_like(hello));

    let hello_world = b"hello world";
    assert_eq!(false, is_argument_like(hello));
    assert_eq!(false, is_subcommand_like(hello));
    assert_eq!(false, is_path_like(hello));

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
    for byte in s {
        if matches!(byte, b'/' | b'\\' | b'.') {
            return true;
        }
    }
    false
}

fn is_subcommand_like(s: &[u8]) -> bool {
    if s.is_empty() {
        return false;
    }
    for byte in s {
        if !matches!(byte, b'a'..=b'z' | b'_' | b'-' | b'0'..=b'9') {
            return false;
        }
    }
    true
}
