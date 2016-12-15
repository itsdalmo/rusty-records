#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;
extern crate csv;
extern crate regex;
pub mod error;

use regex::Regex;
use std::result;

pub type Result<T> = result::Result<T, error::Error>;

/// Stringify records.
pub fn stringify(v: &Vec<Vec<String>>) -> csv::Writer<Vec<u8>> {
    let mut csv = csv::Writer::from_memory().delimiter(b'|');
    for record in v.into_iter() {
        match csv.write(record.into_iter()) {
            Ok(_)  => {},
            Err(_) => {},
        }
    }
    csv
}

#[test]
fn test_stringify() {
    let v = vec!["1316", "NY21", "ET LITE SÆLSKAP I ÅL", "429", "", "2000-01-01T00:00:00+00:00", "9999-12-31T00:00:00+00:00"];
    let v = vec![v.iter().map(|s| s.to_string()).collect()];
    let r = "1316|NY21|ET LITE SÆLSKAP I ÅL|429||2000-01-01T00:00:00+00:00|9999-12-31T00:00:00+00:00\n";
    assert_eq!(stringify(&v).as_string(), r);
}

/// Handle a line. Reads as CSV and cleans up individual fields.
pub fn handle_line(s: &str) -> csv::Writer<Vec<u8>> {
    let mut csv = csv::Reader::from_string(s).has_headers(false).delimiter(b'|').flexible(true);
    let mut out = csv::Writer::from_memory().delimiter(b'|');
    let mut rec: Vec<String> = vec![];
    loop {
        match csv.next_str() {
            csv::NextField::Data(v) => {
                rec.push(clean_field(v).unwrap_or("".to_string()));
            },
            csv::NextField::EndOfRecord => {
                out.write(rec.clone().into_iter());
                rec.clear();
            },
            csv::NextField::EndOfCsv => break,
            csv::NextField::Error(_) => {},
        }
    }
    out
}

#[test]
fn test_handle_line() {
    let s = "1316|\"NY21\"|\"ET LITE SÆLSKAP I ÅL\"|429|\"\"|2000-01-01T00:00:00+00:00|9999-12-31T00:00:00+00:00\n295916617|640487906|640487906|\"0000000000\"|\"L\"|2016-01-14T00:00:00+00:00|240241235|244592895|2016-01-14T16:54:38+00:00||501687|\"\"|\"CS029\"||||\"GOUDA#BLÅMUGOST VAN CHEDDAR\"|\"1234\"|\"POSTBOKS 565, OBS!!! \"PRIVAT/PERSONLIG\"\"|\"OSLO\"|\"\"|\"P\"||1953-08-14T00:00:00+00:00|\"\"\n295916617|640487906|640487906|\"0000000000\"|\"L\"|2016-01-14T00:00:00+00:00|240241235|244592895|2016-01-14T16:54:38+00:00||501687|\"\"|\"CS029\"||||\"GOUDA#BLÅMUGOST VAN CHEDDAR\"|\"1234\"|\"POSTBOKS 565, OBS!!! \"PRIVAT/PERSONLIG\"|\"OSLO\"|\"\"|\"P\"||1953-08-14T00:00:00+00:00|\"\"";
    let r = vec![
        vec!["1316", "NY21", "ET LITE SÆLSKAP I ÅL", "429", "", "2000-01-01T00:00:00+00:00", "9999-12-31T00:00:00+00:00"],
        vec!["295916617", "640487906", "640487906", "0000000000", "L", "2016-01-14T00:00:00+00:00", "240241235", "244592895", "2016-01-14T16:54:38+00:00", "", "501687", "", "CS029", "", "", "", "GOUDA#BLÅMUGOST VAN CHEDDAR", "1234", "POSTBOKS 565, OBS!!! PRIVAT/PERSONLIG", "OSLO", "", "P", "", "1953-08-14T00:00:00+00:00", ""],
        vec!["295916617", "640487906", "640487906", "0000000000", "L", "2016-01-14T00:00:00+00:00", "240241235", "244592895", "2016-01-14T16:54:38+00:00", "", "501687", "", "CS029", "", "", "", "GOUDA#BLÅMUGOST VAN CHEDDAR", "1234", "POSTBOKS 565, OBS!!! PRIVAT/PERSONLIG", "OSLO", "", "P", "", "1953-08-14T00:00:00+00:00", ""]
    ];
    assert_eq!(handle_line(&s).unwrap(), r);
}

/// Reads string-csv data using BurntSushi's CSV library.
pub fn read_line(s: &str) -> Result<Vec<Vec<String>>> {
    let mut csv = csv::Reader::from_string(s).has_headers(false).delimiter(b'|').flexible(true);
    let mut res: Vec<Vec<String>> = vec![];
    for line in csv.records() {
        match line {
            Ok(v)  => res.push(v),
            Err(_) => {},
        }
    }
    Ok(res)
}

#[test]
fn test_read_line() {
    let s = "1316|\"NY21\"|\"ET LITE SÆLSKAP I ÅL\"|429|\"\"|2000-01-01T00:00:00+00:00|9999-12-31T00:00:00+00:00\n";
    let r = vec![vec!["1316", "NY21", "ET LITE SÆLSKAP I ÅL", "429", "", "2000-01-01T00:00:00+00:00", "9999-12-31T00:00:00+00:00"]];
    assert_eq!(read_line(&s).unwrap(), r);
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

