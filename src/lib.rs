#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;
extern crate csv;
extern crate regex;
pub mod error;

use regex::Regex;
use std::result;

pub type Result<T> = result::Result<T, error::Error>;

/// Pass lines from stdin to stdout
pub fn pass(r: &mut csv::Reader<std::io::Stdin>, w: &mut csv::Writer<std::io::Stdout>) {
    for record in r.records() {
        let rec = record.unwrap();
        w.write(rec.iter()).unwrap();
    }
}

/// Handle lines from stdin and writes to stdout. Cleans up fields in all records.
pub fn handle_lines(r: &mut csv::Reader<std::io::Stdin>, w: &mut csv::Writer<std::io::Stdout>) {
    for record in r.records() {
        let rec = record.unwrap();
        let rec: Vec<String> = rec.iter().map(|f| clean_field(f).unwrap_or("".to_string())).collect();
        w.write(rec.iter()).unwrap();
    }
}

/// Cleans a single field of quotes, linebreaks and trailing whitespace.
pub fn clean_field(s: &str) -> Result<String> {
    let s = try!(remove_quotes(&s));
    let s = try!(replace_linebreaks(&s));
    let s = try!(trim_right(&s));
    Ok(s)
}

#[test]
fn test_clean_field() {
    let s = "POSTBOKS 565,\n\"OBS!!!!\"  ";
    assert_eq!(clean_field(&s).unwrap(), "POSTBOKS 565, OBS!!!!");
}

/// Remove quotes from a field in a record (" and ').
fn remove_quotes(s: &str) -> Result<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new("[\"']+").unwrap();
    }
    Ok(RE.replace_all(s, ""))
}

#[test]
fn test_remove_quotes() {
    let s = "POSTBOKS 565, OBS!!! PRIVAT/PERSONLIG\"\"";
    assert_eq!(remove_quotes(&s).unwrap(), "POSTBOKS 565, OBS!!! PRIVAT/PERSONLIG");
}

/// Replaces one or more linebreaks in a field with a single space.
fn replace_linebreaks(s: &str) -> Result<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new("[\n\r]+").unwrap();
    }
    Ok(RE.replace_all(s, " "))
}

#[test]
fn test_replace_linebreaks() {
    let s = "POSTBOKS 565, \n, \n\r, TEST";
    assert_eq!(replace_linebreaks(&s).unwrap(), "POSTBOKS 565,  ,  , TEST");
}

/// Removes trailing whitespace (including linebreaks and tabs) for a field.
fn trim_right(s: &str) -> Result<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new("[:space:]+$").unwrap();
    }
    Ok(RE.replace_all(s, ""))
}

#[test]
fn test_trim_right() {
    let s = "POSTBOKS 565,   \n";
    assert_eq!(trim_right(&s).unwrap(), "POSTBOKS 565,");
}

