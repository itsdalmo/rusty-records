#![feature(test)]

extern crate rusty_records;
extern crate test;

use test::Bencher;

#[bench]
fn bench_stringify(b: &mut Bencher) {
    let v = vec!["1316", "NY21", "ET LITE SÆLSKAP I ÅL", "429", "", "2000-01-01T00:00:00+00:00", "9999-12-31T00:00:00+00:00"];
    let v = vec![v.iter().map(|s| s.to_string()).collect()];
    b.iter(|| rusty_records::stringify(&v));
}

#[bench]
fn bench_handle_line(b: &mut Bencher) {
    let s = "1316|\"NY21\"|\"ET LITE SÆLSKAP I ÅL\"|429|\"\"|2000-01-01T00:00:00+00:00|9999-12-31T00:00:00+00:00\n";
    b.iter(|| rusty_records::handle_line(&s));
}

#[bench]
fn bench_clean_field(b: &mut Bencher) {
    let s = "\"1234\"   \n";
    b.iter(|| rusty_records::clean_field(&s));
}

#[bench]
fn bench_read_line(b: &mut Bencher) {
    let s = "1316|\"NY21\"|\"ET LITE SÆLSKAP I ÅL\"|429|\"\"|2000-01-01T00:00:00+00:00|9999-12-31T00:00:00+00:00\n";
    b.iter(|| rusty_records::read_line(&s));
}
