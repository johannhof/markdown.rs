mod span;
mod block;

#[allow(missing_docs)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OrderedListType(pub String);

#[allow(missing_docs)]
#[derive(Debug, PartialEq, Clone)]
pub enum Block {
    Header(Vec<Span>, usize),
    Paragraph(Vec<Span>),
    Blockquote(Vec<Block>),
    CodeBlock(Option<String>, String),
    //String is the type of list: A,a,i,I or 1
    OrderedList(Vec<ListItem>,OrderedListType),
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
    Link(Vec<Span>, String, Option<String>),
    Image(String, String, Option<String>),

    Emphasis(Vec<Span>),
    Strong(Vec<Span>)
}

pub fn parse (md : &str) -> Vec<Block> {
    block::parse_blocks(md)
}

