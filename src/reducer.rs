#[macro_use]
extern crate clap;
extern crate csv;
extern crate rusty_records;

use clap::App;
use std::io::{self};


fn main() {
    let _ = App::new("EMR Reducer")
        .usage("cat <INPUT> | reducer > <OUTPUT>")
        .version(crate_version!())
        .author(crate_authors!())
        .about("http://github.com/itsdalmo/rusty-records")
        .get_matches();

    let mut rdr = csv::Reader::from_reader(io::stdin()).has_headers(false).delimiter(b'|').flexible(true);
    let mut wrt = csv::Writer::from_writer(io::stdout()).delimiter(b'|').flexible(true);
    rusty_records::pass(&mut rdr, &mut wrt);
}
