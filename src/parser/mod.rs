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
    CodeBlock(Option<&'a str>, Vec<&'a str>),
    /** A link reference with the fields: (id, url, [title]) **/
    LinkReference(String, &'a str, Option<&'a str>),
    OrderedList(Vec<ListItem<'a>>, OrderedListType),
    UnorderedList(Vec<ListItem<'a>>),
    Raw(&'a str),
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
    Text(&'a str),
    Code(&'a str),
    Literal(char),
    Link(Vec<Span<'a>>, &'a str, Option<&'a str>),
    /**
     * A reference-style link with the fields: (content, url, raw)
     * The "raw" field is used internally for falling back to the original
     * markdown link if the corresponding reference is not found at render time.
     **/
    RefLink(Vec<Span<'a>>, String, &'a str),
    Image(&'a str, &'a str, Option<&'a str>),

    Emphasis(Vec<Span<'a>>),
    Strong(Vec<Span<'a>>),
}

pub fn parse(md: &str) -> Vec<Block> {
    block::parse_blocks(md)
}
