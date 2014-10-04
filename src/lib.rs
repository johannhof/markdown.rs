#![crate_name = "markdown"]

#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

mod parser;
mod html;
mod normalizer;

pub fn to_html(text : &str) -> String{
    let normalized = normalizer::normalize(text.as_slice());
    let result = parser::parse(normalized.as_slice());
    html::to_html(result)
}

