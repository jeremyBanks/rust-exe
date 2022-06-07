#!/usr/bin/env rust
fn main() {
    let working_dir = ::std::env::current_dir().unwrap().file_name().unwrap().to_str().unwrap().to_string();
    let current_exe = ::std::env::current_exe().unwrap().file_name().unwrap().to_str().unwrap().to_string();
    let args = Vec::from_iter(::std::env::args().skip(1));
    dbg!(working_dir, current_exe, args);
}
