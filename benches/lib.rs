
#![feature(test)]

extern crate markdown;
extern crate test;

use test::Bencher;
use std::fs;

// Current benchmark: 3,055,292 ns/iter (+/- 328,418)
#[bench]
fn parse_markdown_file(b: &mut Bencher) {
    let md = fs::read_to_string("benches/files/Markdown.md").unwrap();
    b.iter(|| markdown::tokenize(&md));
}