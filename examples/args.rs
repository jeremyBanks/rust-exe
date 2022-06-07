#!/usr/bin/env rust
fn main() {
    let working_dir = ::std::env::current_dir().unwrap();
    let current_exe = ::std::env::current_exe().unwrap();
    let args = Vec::from_iter(::std::env::args().skip(1));
    dbg!(working_dir, current_exe, args);
}
