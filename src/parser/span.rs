use regex::Regex;
use parser::{Span, Text, Emphasis, Strong, Code, Link, Image};

static SPANS : Regex = regex!(r"(!?\[.*\]\([^\(\)]*\))|(\*[^\*].+?\*)|(\*\*.+?\*\*)|(_[^_].+?_)|(__.+?__)|(`[^`].+?`)|(``.+?``)");
static LINK  : Regex = regex!("\\[(?P<text>.*)\\]\\((?P<url>.*?)(?:\\s\"(?P<title>.*?)\")?\\)");
static IMAGE  : Regex = regex!("!\\[(?P<text>.*)\\]\\((?P<url>.*?)(?:\\s\"(?P<title>.*?)\")?\\)");
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

fn parse_span(text : &str) -> Span{
    if IMAGE.is_match(text){
        let caps = IMAGE.captures(text).unwrap();
        return Image(
            caps.name("text"),
            caps.name("url"),
            caps.name("title")
            );
    }else if LINK.is_match(text){
        let caps = LINK.captures(text).unwrap();
        return Link(
            caps.name("text"),
            caps.name("url"),
            caps.name("title")
            );
    }else if EMPHASIS_UNDERSCORE.is_match(text){
        let caps = EMPHASIS_UNDERSCORE.captures(text).unwrap();
        return Emphasis(parse_spans(caps.name("text")));
    }else if EMPHASIS_STAR.is_match(text){
        let caps = EMPHASIS_STAR.captures(text).unwrap();
        return Emphasis(parse_spans(caps.name("text")));
    }
    return Text(text);
}

// HERE BE TEST

#[test]
fn parse_link_test() {
    assert_eq!(parse_span("[an example](example.com)"),
                Link("an example", "example.com", ""));

    assert_eq!(parse_span("[an example](example.com \"Title\")"),
                Link("an example", "example.com", "Title"));
}

#[test]
fn parse_image_test() {
    assert_eq!(parse_span("![an example](example.com)"),
                Image("an example", "example.com", ""));
    assert_eq!(parse_span("![an example](example.com \"Title\")"),
                Image("an example", "example.com", "Title"));
}


#[test]
fn parse_emphasis_test() {

    // simple text behaviour
    assert_eq!(parse_span("_whatever_"), Emphasis(vec![Text("whatever")]));
    assert_eq!(parse_span("*whatever*"), Emphasis(vec![Text("whatever")]));

    // multiple words
    assert_eq!(
        parse_span("_markdown is better than nothing_"),
        Emphasis(vec![Text("markdown is better than nothing")])
              );
    assert_eq!(
        parse_span("*markdown is better than nothing*"),
        Emphasis(vec![Text("markdown is better than nothing")])
              );

    // multiple words
    assert_eq!(
        parse_span("_[an example](example.com) is better than nothing_"),
        Emphasis(vec![
                 Link("an example", "example.com", ""),
                 Text(" is better than nothing")
                 ])
              );

    assert_eq!(
        parse_span("*[an example](example.com) is better than nothing*"),
        Emphasis(vec![
                 Link("an example", "example.com", ""),
                 Text(" is better than nothing")
                 ])
              );

    // does not compile strong
    match parse_span("__whatever__"){
      Emphasis(_) => fail!(),
      _ => {}
    }
    match parse_span("**whatever**"){
      Emphasis(_) => fail!(),
      _ => {}
    }
}

//#[test]
//#[ignore]
//fn parse_strong_test() {
    //match parse_span("__whatever__"){
      //Strong("whatever") => {},
      //_ => fail!()
    //}
    //match parse_span("__what_ever__"){
      //Strong("what_ever") => {},
      //_ => fail!()
    //}
    //match parse_span("_whatever_"){
      //Strong("whatever") => fail!(),
      //_ => {}
    //}
    //match parse_span("**whatever**"){
      //Strong("whatever") => {},
      //_ => fail!()
    //}
    //match parse_span("**what*ever**"){
      //Strong("what*ever") => {},
      //_ => fail!()
    //}
    //match parse_span("*whatever*"){
      //Strong("whatever") => fail!(),
      //_ => {}
    //}
//}

