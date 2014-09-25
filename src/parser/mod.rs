use regex::Regex;

mod span;
mod block;

static SPLIT : Regex = regex!(r"\n\n");

#[deriving(Show)]
pub enum Block<'s> {
    Header(Vec<Span<'s>>, uint),
    Break,
    Paragraph(Vec<Span<'s>>)
}

#[deriving(Show)]
pub enum Span<'s> {
    Text(&'s str),
    Link(&'s str, &'s str, &'s str),
    Image(&'s str, &'s str, &'s str),

    Emphasis(Vec<Span<'s>>),
    Strong(Vec<Span<'s>>),
    Code(Vec<Span<'s>>)
}

pub fn parse (md : &str) -> Vec<Block> {
    let mut split = SPLIT.split(md);
    let mut tokens = vec![];
    for block in split{
        match block::parse_block(block) {
            Some(e) => tokens.push(e),
            None => {},
        };
    }
    tokens
}

