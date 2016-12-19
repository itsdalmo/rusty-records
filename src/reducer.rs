#[macro_use]
extern crate clap;
extern crate csv;
extern crate rusty_records;

use clap::{App, Arg};


fn main() {
    let matches = App::new("EMR Reducer")
        .usage("cat <INPUT> | reducer > <OUTPUT>")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(Arg::with_name("input")
             .long("input")
             .short("i")
             .value_name("FILE")
             .help("Path to input. Defaults to STDIN."))
        .arg(Arg::with_name("output")
             .long("output")
             .short("o")
             .value_name("FILE")
             .help("Path to output. Defaults to STDOUT."))
        .about("http://github.com/itsdalmo/rusty-records")
        .get_matches();

    let input: Box<std::io::Read> = match matches.value_of("input") {
        Some(v) => Box::new(std::fs::File::open(v).expect("Failed to open input.")),
        None    => Box::new(std::io::stdin()),
    };

    let output: Box<std::io::Write> = match matches.value_of("output") {
        Some(v) => Box::new(std::fs::OpenOptions::new()
                            .create(true).write(true)
                            .open(v)
                            .expect("Failed to open output.")),
        None    => Box::new(std::io::stdout()),
    };

    let mut rdr = csv::Reader::from_reader(input).has_headers(false).delimiter(b'|').flexible(true);
    let mut wrt = csv::Writer::from_writer(output).delimiter(b'|').flexible(true);
    rusty_records::pass(&mut rdr, &mut wrt);
}
