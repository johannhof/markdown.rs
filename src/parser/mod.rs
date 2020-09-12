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
pub enum Block {
    Header(Vec<Span>, usize),
    Paragraph(Vec<Span>),
    Blockquote(Vec<Block>),
    CodeBlock(Option<String>, String),
    /** A link reference with the fields: (id, url, [title]) **/
    LinkReference(String, String, Option<String>),
    OrderedList(Vec<ListItem>, OrderedListType),
    UnorderedList(Vec<ListItem>),
    Raw(String),
    Hr,
}

#[allow(missing_docs)]
#[derive(Debug, PartialEq, Clone)]
pub enum ListItem {
    Simple(Vec<Span>),
    Paragraph(Vec<Block>),
}

#[allow(missing_docs)]
#[derive(Debug, PartialEq, Clone)]
pub enum Span {
    Break,
    Text(String),
    Code(String),
    Literal(char),
    Link(Vec<Span>, String, Option<String>),
    /**
     * A reference-style link with the fields: (content, url, raw)
     * The "raw" field is used internally for falling back to the original
     * markdown link if the corresponding reference is not found at render time.
     **/
    RefLink(Vec<Span>, String, String),
    Image(String, String, Option<String>),

    Emphasis(Vec<Span>),
    Strong(Vec<Span>),
}

pub fn parse(md: &str) -> Vec<Block> {
    block::parse_blocks(md)
}
