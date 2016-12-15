extern crate csv;
extern crate rusty_records;

use std::io::{self, Write, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let mut out = rusty_records::handle_line(&line.unwrap());
        io::stdout().write(out.as_bytes()).unwrap();
    }
}
