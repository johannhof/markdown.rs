use regex::Regex;
use parser::{Span, Text, Emphasis, Strong, Code, Link, Image};

// Span Patters
static SPANS : Regex = regex!(r"(!?\[.*\]\([^\(\)]*\))|(\*[^\*].+?\*)|(\*\*.+?\*\*)|(_[^_].+?_)|(__.+?__)|(`[^`].+?`)|(``.+?``)");
static LINK  : Regex = regex!("\\[(?P<text>.*)\\]\\((?P<url>.*?)(?:\\s\"(?P<title>.*?)\")?\\)");
static EMPHASIS_UNDERSCORE  : Regex = regex!(r"^_(?P<text>[^_].+?)_");
static EMPHASIS_STAR  : Regex = regex!(r"^\*(?P<text>[^\*].+?)\*");

pub fn parse_spans(text : &str) -> Vec<Span>{
    let mut tokens = vec![];
    let mut current = 0;
    for (begin, end) in SPANS.find_iter(text) {
        tokens.push(Text(text.slice(current, begin)));
        tokens.push(parse_span(text.slice(begin, end)));
        current = end;
    }
    tokens.push(Text(text.slice(current, text.len())));
    tokens
}

#[test]
fn parse_emphasis_test() {
    match parse_span("_whatever_"){
      Emphasis("whatever") => {},
      _ => fail!()
    }
    match parse_span("__whatever__"){
      Emphasis("whatever") => fail!(),
      _ => {}
    }
}

#[test]
fn parse_link_test() {
    match parse_span("[an example](example.com)"){
      Link("an example", "example.com", "") => {},
      _ => fail!()
    }
    match parse_span("[an example](example.com \"Title\")"){
      Link("an example", "example.com", "Title") => assert!(true),
      _ => fail!()
    }
}

fn parse_span(text : &str) -> Span{
    if LINK.is_match(text){
        let caps = LINK.captures(text).unwrap();
        return Link(
            caps.name("text"),
            caps.name("url"),
            caps.name("title")
            );
    }else if EMPHASIS_UNDERSCORE.is_match(text){
        let caps = EMPHASIS_UNDERSCORE.captures(text).unwrap();
        return Emphasis(caps.name("text"));
    }else if EMPHASIS_STAR.is_match(text){
        let caps = EMPHASIS_STAR.captures(text).unwrap();
        return Emphasis(caps.name("text"));
    }
    return Text(text);
}
