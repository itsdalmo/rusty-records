#[macro_use]
extern crate lazy_static;
extern crate csv;
extern crate regex;
extern crate rayon;
pub mod error;

use rayon::prelude::*;
use regex::Regex;
use std::result;

pub type Result<T> = result::Result<T, error::Error>;

/// Pass lines from stdin to stdout
pub fn pass(r: &mut csv::Reader<Box<std::io::Read>>, w: &mut csv::Writer<Box<std::io::Write>>) {
    r.byte_records().par_iter(|record| {
        if let Ok(rec) = record {
            let _ = w.write(rec.iter());
        }
    });
}

/// Handle lines from stdin and writes to stdout. Cleans up fields in all records.
pub fn handle_lines(r: &mut csv::Reader<Box<std::io::Read>>, w: &mut csv::Writer<Box<std::io::Write>>) {
    r.records().par_iter(|record| {
        if let Ok(rec) = record {
            let rec = rec.iter().map(|f| clean_field(f).unwrap_or("".to_string()));
            let _ = w.write(rec);
        }
    });
}

/// Cleans a single field of quotes, linebreaks and trailing whitespace.
pub fn clean_field(s: &str) -> Result<String> {
    let s = remove_quotes(&s)?;
    let s = replace_linebreaks(&s)?;
    let s = s.trim().to_string();
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

#[test]
fn test_trim_right() {
    let s = "POSTBOKS 565,   \n";
    assert_eq!(s.trim_right(), "POSTBOKS 565,");
}

