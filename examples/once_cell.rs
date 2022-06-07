#!/usr/bin/env rust
use ::once_cell::sync::OnceCell;

pub fn main() {
    static CELL: OnceCell<&str> = OnceCell::new();
    CELL.set("hello, rust").unwrap();
    println!("{}", CELL.get_or_init(|| "goodbye, rust"));
}
