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
