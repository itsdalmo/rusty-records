extern crate csv;
extern crate rusty_records;

use std::io::{self, Write, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let out = rusty_records::handle_line(&line.unwrap());
        let mut out = rusty_records::stringify(out.unwrap());
        io::stdout().write(out.as_bytes()).unwrap();
    }
}
