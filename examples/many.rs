#!/usr/bin/env rust
fn main() {
    let quote = "that's one small step for [a] man";

    let quote = ::base64::encode(quote).to_string();

    let quote = ::heck::AsTitleCase(quote).to_string();

    println!("{quote}");
}
