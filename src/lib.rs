#![crate_name = "markdown"]

#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

use std::io::File;
use std::io::IoError;

mod parser;
mod html;
mod normalizer;

pub fn to_html(text : &str) -> String{
    let normalized = normalizer::normalize(text.as_slice());
    let result = parser::parse(normalized.as_slice());
    html::to_html(&result)
}

pub fn file_to_html(path : &Path) -> Result<String, IoError>{
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let text = match file.read_to_string() {
        Ok(string) => string,
        Err(e) => return Err(e)
    };

    let normalized = normalizer::normalize(text.as_slice());
    let result = parser::parse(normalized.as_slice());
    Ok(html::to_html(&result))
}

