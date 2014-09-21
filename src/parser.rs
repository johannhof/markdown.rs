use regex::Regex;

#[deriving(Show)]
pub enum Block<'s> {
    Header(Vec<Span<'s>>, uint),
    Break,
    Paragraph(Vec<Span<'s>>)
}

#[deriving(Show)]
pub enum Span<'s> {
    Text(&'s str),
    Emphasis(&'s str),
    Strong(&'s str),
    Code(&'s str),
    Link(&'s str, &'s str, &'s str),
    Image(&'s str, &'s str, &'s str)
}

// Block Patters
static SPLIT           : Regex = regex!(r"\n\n");
static ATX_HEADER      : Regex = regex!(r"(?P<level>#{1,6})\s(?P<text>.*)");
static SETEXT_HEADER_1 : Regex = regex!(r"(?P<text>.+)\n===+");
static SETEXT_HEADER_2 : Regex = regex!(r"(?P<text>.+)\n---+");
static BREAK           : Regex = regex!(r"  ");

// Span Patters
static SPANS : Regex = regex!(r"(!?\[.*\]\([^\(\)]*\))|(\*[^\*].+?\*)|(\*\*.+?\*\*)|(_[^_].+?_)|(__.+?__)|(`[^`].+?`)|(``.+?``)");
static LINK  : Regex = regex!(r"\[(?P<text>.*)\]\((?P<url>.*?)(?:\s(?P<title>.*?))?\)");

pub fn parse (md : &str) -> Vec<Block> {
    let mut split = SPLIT.split(md);
    let mut tokens = vec![];
    for block in split{
        match parse_block(block) {
            Some(e) => tokens.push(e),
            None => {},
        };
    }
    tokens
}

fn parse_spans(text : &str) -> Vec<Span>{
    let mut tokens = vec![];
    let mut current = 0;
    for (begin, end) in SPANS.find_iter(text) {
        tokens.push(Text(text.slice(current, begin)));
        tokens.push(parse_span(text.slice(begin, end)));
        current = end;
    }
    tokens
}

fn parse_span(text : &str) -> Span{
    if LINK.is_match(text){
        let caps = LINK.captures(text).unwrap();
        return Link(
            caps.name("text"),
            caps.name("url"),
            caps.name("title")
            );
    }
    return Text(text);
}

fn parse_block (text : &str) -> Option<Block>{
    if text.is_empty(){
        return None;
    }else if ATX_HEADER.is_match(text){
        let caps = ATX_HEADER.captures(text).unwrap();
        return Some(
            Header(
                parse_spans(caps.name("text")),
                caps.name("level").len()
                )
            );
    }else if SETEXT_HEADER_1.is_match(text){
        let caps = SETEXT_HEADER_1.captures(text).unwrap();
        return Some(
            Header(
                parse_spans(caps.name("text")),
                1
                )
            );
    }else if SETEXT_HEADER_2.is_match(text){
        let caps = SETEXT_HEADER_2.captures(text).unwrap();
        return Some(
            Header(
                parse_spans(caps.name("text")),
                2
                )
            );
    }else if BREAK.is_match(text){
        return Some(Break);
    }
    return Some(Paragraph(parse_spans(text)));
}

