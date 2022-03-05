mod block;
mod span;

/// The style for ordered list numerals
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum OrderedListType {
    /// ```text
    /// 1. First item
    /// 2. Second item
    /// 3. Third item
    /// ```
    Numeric,
    /// ```text
    /// a. First item
    /// b. Second item
    /// c. Third item
    /// ```
    Lowercase,
    /// ```text
    /// A. First item
    /// B. Second item
    /// C. Third item
    /// ```
    Uppercase,
    /// ```text
    /// i. First item
    /// ii. Second item
    /// iii. Third item
    /// iv. Fourth item
    /// ```
    LowercaseRoman,
    /// ```text
    /// I. First item
    /// II. Second item
    /// III. Third item
    /// IV. Fourth item
    /// ```
    UppercaseRoman,
}

impl OrderedListType {
    fn from_str(type_str: &str) -> OrderedListType {
        match type_str {
            "a" => OrderedListType::Lowercase,
            "A" => OrderedListType::Uppercase,
            "i" => OrderedListType::LowercaseRoman,
            "I" => OrderedListType::UppercaseRoman,
            _ => OrderedListType::Numeric,
        }
    }

    /// Converts the ordered list type into the corresponding <ol> "type"
    /// attribute value.
    pub fn to_html_type(&self) -> &'static str {
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
