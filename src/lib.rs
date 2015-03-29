#![crate_name = "markdown"]

#![feature(fs)]
#![feature(io)]
#![feature(plugin)]
#![plugin(regex_macros)]
extern crate regex_macros;
extern crate regex;

use std::fs::File;
use std::path::Path;
use std::io::{Read, Error};

mod parser;
mod html;
mod normalizer;

pub fn to_html(text : &str) -> String{
    let normalized = normalizer::normalize(&text);
    let result = parser::parse(&normalized);
    html::to_html(&result)
}

pub fn file_to_html(path : &Path) -> Result<String, Error>{
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Ok(_) => (),
        Err(e) => return Err(e)
    };

    let normalized = normalizer::normalize(&text);
    let result = parser::parse(&normalized);
    Ok(html::to_html(&result))
}

