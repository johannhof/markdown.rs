//! A crate for parsing Markdown in Rust
#![crate_name = "markdown"]
#![deny(missing_docs)]
// #![deny(warnings)]

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate regex;

#[macro_use]
extern crate pipeline;

use std::fs::File;
use std::path::Path;
use std::io::{self, Read};

mod parser;
mod html;

use parser::Block;

/// Converts a Markdown string to HTML
pub fn to_html(text: &str) -> String {
    let result = parser::parse(text);
    html::to_html(&result)
}

/// Converts a Markdown string to a tokenset of Markdown items
pub fn tokenize(text: &str) -> Vec<Block> {
    parser::parse(text)
}

/// Opens a file and converts its contents to HTML
pub fn file_to_html(path: &Path) -> io::Result<String> {
    let mut file = try!(File::open(path));

    let mut text = String::new();
    try!(file.read_to_string(&mut text));

    let result = parser::parse(&text);
    Ok(html::to_html(&result))
}
