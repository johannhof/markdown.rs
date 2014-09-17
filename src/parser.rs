//extern crate regex;
//use regex::Regex;

pub struct Token {
    kind : String
}

pub fn parse (md : &'static str) -> Vec<Token> {
    let text = String::from_str(md);
    let mut tokens = vec![];
    tokens
}

