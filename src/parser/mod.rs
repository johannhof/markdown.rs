use std::borrow::Cow;

mod block;
mod span;

#[allow(missing_docs)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum OrderedListType {
    Numeric,
    Lowercase,
    Uppercase,
    LowercaseRoman,
    UppercaseRoman,
}

impl OrderedListType {
    pub fn from_str(type_str: &str) -> OrderedListType {
        match type_str {
            "a" => OrderedListType::Lowercase,
            "A" => OrderedListType::Uppercase,
            "i" => OrderedListType::LowercaseRoman,
            "I" => OrderedListType::UppercaseRoman,
            _ => OrderedListType::Numeric,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            OrderedListType::Lowercase => "a",
            OrderedListType::Uppercase => "A",
            OrderedListType::LowercaseRoman => "i",
            OrderedListType::UppercaseRoman => "I",
            OrderedListType::Numeric => "1",
        }
    }
}

#[allow(missing_docs)]
#[derive(Debug, PartialEq, Clone)]
pub enum Block<'a> {
    Header(Vec<Span<'a>>, usize),
    Paragraph(Vec<Span<'a>>),
    Blockquote(Vec<Block<'a>>),
    CodeBlock(Option<Cow<'a, str>>, Vec<&'a str>),
    /** A link reference with the fields: (id, url, [title]) **/
    LinkReference(String, String, Option<String>),
    OrderedList(Vec<ListItem<'a>>, OrderedListType),
    UnorderedList(Vec<ListItem<'a>>),
    Raw(String),
    Hr,
}

#[allow(missing_docs)]
#[derive(Debug, PartialEq, Clone)]
pub enum ListItem<'a> {
    Simple(Vec<Span<'a>>),
    Paragraph(Vec<Block<'a>>),
}

#[allow(missing_docs)]
#[derive(Debug, PartialEq, Clone)]
pub enum Span<'a> {
    Break,
    Text(Cow<'a, str>),
    Code(String),
    Literal(char),
    Link(Vec<Span<'a>>, String, Option<String>),
    /**
     * A reference-style link with the fields: (content, url, raw)
     * The "raw" field is used internally for falling back to the original
     * markdown link if the corresponding reference is not found at render time.
     **/
    RefLink(Vec<Span<'a>>, String, String),
    Image(String, String, Option<String>),

    Emphasis(Vec<Span<'a>>),
    Strong(Vec<Span<'a>>),
}

pub fn parse(md: &str) -> Vec<Block> {
    block::parse_blocks(md)
}
