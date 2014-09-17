#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

mod parser;

fn main() {
    let text = " #### Hello World
                    ## Also
                    Stuff
                    ";
    let result = parser::parse(text);
    println!("{}", result);
}

