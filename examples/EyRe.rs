#!/usr/bin/env rust
extern crate eyre;
pub fn main() -> eyre::Result<()> {
    let mut numbers: Vec<f64> = Vec::new();
    for arg in ::std::env::args().skip(1) {
        numbers.push(arg.parse()?);
    }
    println!("{numbers:?}");
    Ok(())
}
