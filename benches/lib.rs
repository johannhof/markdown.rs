
#![feature(test)]

extern crate markdown;
extern crate test;

use test::Bencher;
use std::fs;

#[bench]
fn parse_markdown_file(b: &mut Bencher) {
    let md = fs::read_to_string("benches/files/Markdown.md").unwrap();

    // Force all the lazy_static's to initialize
    test::black_box(markdown::tokenize(&md));
    
    b.iter(|| markdown::tokenize(&md));
}

/// This is a sanity check. It should be roughly 100x the run time of parse_markdown_file,
/// UNLESS some n^2 algorithm snuck into the parser. Other than that, it's not very useful;
/// it's no more accurate or precise than parse_markdown_file.
/// 
/// Due to the large number of allocations involved, it should be slighly more expensive since
/// we're applying more stress to the allocator. In practice we should expect roughly 110x the
/// run time of parse_markdown_file.
#[bench]
fn parse_100_markdown_files_concatenated(b: &mut Bencher) {
    let md = fs::read_to_string("benches/files/Markdown.md").unwrap();
    let mut concat = String::with_capacity((md.len() + 1) * 100);
    for _ in 0..100 {
        concat.push_str(&md);
        concat.push('\n');
    }

    // Force all the lazy_static's to initialize
    test::black_box(markdown::tokenize(&md));
    
    b.iter(|| markdown::tokenize(&concat));
}