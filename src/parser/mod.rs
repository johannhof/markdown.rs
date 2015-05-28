
use regex::Regex;

mod span;
mod block;

#[derive(Debug, PartialEq)]
pub enum Block {
    Header(Vec<Span>, usize),
    Paragraph(Vec<Span>),
    Blockquote(Vec<Block>),
    CodeBlock(String),
    List(Vec<Vec<Span>>),
    Hr
}

#[derive(Debug, PartialEq)]
pub enum Span {
    Break,
    Text(String),
    Code(String),
    Link(String, String, String),
    Image(String, String, String),

    Emphasis(Vec<Span>),
    Strong(Vec<Span>)
}

pub fn parse (md : &str) -> Vec<Block> {
    let SPLIT : Regex = Regex::new(r"\n(?:\s*\n|$)").unwrap();
    let split = SPLIT.split(md);
    let mut tokens = vec![];
    for block in split{
        match block::parse_block(block) {
            Some(e) => tokens.push(e),
            None => {},
        };
    }
    tokens
}


