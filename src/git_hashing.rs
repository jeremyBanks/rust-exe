#![allow(unused)]
use ::{
    sha1::{Digest, Sha1},
    std::fmt::Write,
};

// If we want a static hash representing a tree of files with mtimes, maybe we could
// construct it as though we have a git repo with those files committed as those times,
// for consistency.
// Although this might be a lot of pointless complexity that should be deferred.

static GIT_USER_NAME: &str = "rust-exe[bot]";
static GIT_USER_EMAIL: &str = "107450506+rust-exe[bot]@users.noreply.github.com";

pub fn git_blob_sha1_hex(content: &[u8]) -> String {
    let mut hasher = Sha1::default();
    hasher.update(b"blob ");
    hasher.update(content.len().to_string());
    hasher.update(b"\0");
    hasher.update(content);
    let result = hasher.finalize();
    let mut hex = String::with_capacity(40);
    for byte in result {
        write!(hex, "{:02x}", byte).unwrap();
    }
    hex
}
