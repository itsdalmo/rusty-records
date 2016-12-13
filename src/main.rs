extern crate csv;
extern crate clap;
extern crate rusty_records;

use clap::{App, Arg};
use std::io::{self, Write, BufRead};
use std::process;

fn main() {
    let matches = App::new("rusty-records")
      .version("0.1.0")
      .author("Kristian D. Olsen <kristian@doingit.no>")
      .about("ETL cleaning on Amazon EMR.")
      .arg(Arg::with_name("mode")
           .short("m")
           .long("mode")
           .value_name("MODE")
           .help("Operation mode. Either as mapper or reducer.")
           .takes_value(true)
           .required(true))
      .get_matches();

    let mode  = matches.value_of("mode").unwrap();

    match mode {
        "mapper"  => {
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                let out = rusty_records::handle_line(&line.unwrap());
                let mut out = rusty_records::stringify(out.unwrap());
                io::stdout().write(out.as_bytes()).unwrap();
            }
        },
        "reducer" => {
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                let out = line.unwrap() + "\n";
                io::stdout().write(out.as_bytes()).unwrap();
            }
        },
        _         => {
            println!("--mode must be either mapper or reducer.");
            process::exit(1);
        },
    }
}
