mod span;
mod block;

#[allow(missing_docs)]
#[derive(Debug, PartialEq, Clone)]
pub enum Block {
    Header(Vec<Span>, usize),
    Paragraph(Vec<Span>),
    Blockquote(Vec<Block>),
    CodeBlock(String),
    //OrderedList(Vec<ListItem>),
    UnorderedList(Vec<ListItem>),
    Raw(String),
    Hr
}

#[allow(missing_docs)]
#[derive(Debug, PartialEq, Clone)]
pub enum ListItem {
    Simple(Vec<Span>),
    Paragraph(Vec<Block>)
}

#[allow(missing_docs)]
#[derive(Debug, PartialEq, Clone)]
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

