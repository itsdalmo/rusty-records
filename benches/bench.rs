#![feature(test)]

extern crate rusty_records;
extern crate test;

use test::Bencher;

#[bench]
fn bench_clean_field(b: &mut Bencher) {
    let s = "\"1234\"   \n";
    b.iter(|| rusty_records::clean_field(&s));
}

