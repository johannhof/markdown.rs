mod span;
mod block;

#[derive(Debug, PartialEq)]
pub enum Block {
    Header(Vec<Span>, usize),
    Paragraph(Vec<Span>),
    Blockquote(Vec<Block>),
    CodeBlock(String),
    List(Vec<(Vec<Span>, usize)>),
    Raw(String),
    Hr
}

#[derive(Debug, PartialEq)]
pub enum Span {
    Break,
    Text(String),
    Code(String),
    Link(String, String, Option<String>),
    Image(String, String, Option<String>),

    Emphasis(Vec<Span>),
    Strong(Vec<Span>)
}

pub fn parse (md : &str) -> Vec<Block> {
    block::parse_blocks(md)
}

