use crate::*;
use ::{
    sha1::{Digest, Sha1},
    std::fmt::Write,
};

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
