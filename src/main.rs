#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

use std::io::File;

mod parser;
mod html;
mod normalizer;


fn main() {
    let path = Path::new("test.md");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => fail!("couldn't open {}: {}", display, why.desc),
        Ok(file) => file,
    };

    let text = match file.read_to_string() {
        Err(why) => fail!("couldn't read {}: {}", display, why.desc),
        Ok(string) => string
    };

    let normalized = normalizer::normalize(text.as_slice());
    println!("{}", normalized);
    let result = parser::parse(normalized.as_slice());
    println!("{}", html::to_html(result));
}

