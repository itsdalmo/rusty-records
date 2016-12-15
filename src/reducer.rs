extern crate csv;
extern crate rusty_records;

use std::io::{self};

fn main() {
    let mut rdr = csv::Reader::from_reader(io::stdin()).has_headers(false).delimiter(b'|').flexible(true);
    let mut wrt = csv::Writer::from_writer(io::stdout()).delimiter(b'|').flexible(true);
    rusty_records::pass(&mut rdr, &mut wrt);
}
