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
mod markdown_generator;
mod html;

pub use parser::{Block, ListItem, Span};

/// Converts a Markdown string to HTML
pub fn to_html(text: &str) -> String {
    let result = parser::parse(text);
    html::to_html(&result)
}

/// Converts a Markdown string to a tokenset of Markdown items
pub fn tokenize(text: &str) -> Vec<Block> {
    parser::parse(text)
}

/// Convert tokenset of Markdown items back to String
pub fn generate_markdown(x: Vec<Block>) -> String {
    markdown_generator::generate(x)
}

/// Opens a file and converts its contents to HTML
pub fn file_to_html(path: &Path) -> io::Result<String> {
    let mut file = try!(File::open(path));

    let mut text = String::new();
    try!(file.read_to_string(&mut text));

    let result = parser::parse(&text);
    Ok(html::to_html(&result))
}
