use regex::Regex;
use parser::{Span, Text, Emphasis, Strong, Code, Link, Image};

static SPANS : Regex = regex!(r"(!?\[.*\]\([^\(\)]*\))|(\*[^\*].+?\*)|(\*\*.+?\*\*)|(_[^_].+?_)|(__.+?__)|(`[^`].+?`)|(``.+?``)");
static LINK  : Regex = regex!("\\[(?P<text>.*)\\]\\((?P<url>.*?)(?:\\s\"(?P<title>.*?)\")?\\)");
static IMAGE  : Regex = regex!("!\\[(?P<text>.*)\\]\\((?P<url>.*?)(?:\\s\"(?P<title>.*?)\")?\\)");
static EMPHASIS_UNDERSCORE  : Regex = regex!(r"^_(?P<text>[^_].+?)_");
static EMPHASIS_STAR  : Regex = regex!(r"^\*(?P<text>[^\*].+?)\*");
static STRONG_UNDERSCORE  : Regex = regex!(r"^__(?P<text>.+?)__");
static STRONG_STAR  : Regex = regex!(r"^\*\*(?P<text>.+?)\*\*");
static CODE_SINGLE  : Regex = regex!(r"^`(?P<text>[^`].+?)`");
static CODE_DOUBLE  : Regex = regex!(r"^``(?P<text>.+?)``");

pub fn parse_spans(text : &str) -> Vec<Span>{
    let mut tokens = vec![];
    let mut current = 0;
    for (begin, end) in SPANS.find_iter(text) {
        match text.slice(current, begin){
            "" => {}
            t => tokens.push(Text(t))
        };
        tokens.push(parse_span(text.slice(begin, end)));
        current = end;
    }
    tokens.push(Text(text.slice(current, text.len())));
    tokens
}

fn parse_span(text : &str) -> Span{

    if STRONG_UNDERSCORE.is_match(text){
        let caps = STRONG_UNDERSCORE.captures(text).unwrap();
        return Strong(parse_spans(caps.name("text")));
    }else if STRONG_STAR.is_match(text){
        let caps = STRONG_STAR.captures(text).unwrap();
        return Strong(parse_spans(caps.name("text")));

    }else if EMPHASIS_UNDERSCORE.is_match(text){
        let caps = EMPHASIS_UNDERSCORE.captures(text).unwrap();
        return Emphasis(parse_spans(caps.name("text")));
    }else if EMPHASIS_STAR.is_match(text){
        let caps = EMPHASIS_STAR.captures(text).unwrap();
        return Emphasis(parse_spans(caps.name("text")));

    }else if CODE_DOUBLE.is_match(text){
        let caps = CODE_DOUBLE.captures(text).unwrap();
        return Code(caps.name("text"));
    }else if CODE_SINGLE.is_match(text){
        let caps = CODE_SINGLE.captures(text).unwrap();
        return Code(caps.name("text"));

    }else if IMAGE.is_match(text){
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

    assert_eq!(parse_span("_whatever_"), Emphasis(vec![Text("whatever")]));
    assert_eq!(parse_span("*whatever*"), Emphasis(vec![Text("whatever")]));

    assert_eq!(
        parse_span("_markdown is better than nothing_"),
        Emphasis(vec![Text("markdown is better than nothing")])
              );
    assert_eq!(
        parse_span("*markdown is better than nothing*"),
        Emphasis(vec![Text("markdown is better than nothing")])
              );

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

    assert_eq!(
        parse_span("*[an example](example.com) is _better_ than nothing*"),
        Emphasis(vec![
                 Link("an example", "example.com", ""),
                 Text(" is "),
                 Emphasis(vec![Text("better")]),
                 Text(" than nothing")
                 ])
              );

    assert_eq!(
        parse_span("_[an example](example.com) is *better* than nothing_"),
        Emphasis(vec![
                 Link("an example", "example.com", ""),
                 Text(" is "),
                 Emphasis(vec![Text("better")]),
                 Text(" than nothing")
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

#[test]
fn parse_strong_test() {

    assert_eq!(parse_span("__whatever__"), Strong(vec![Text("whatever")]));
    assert_eq!(parse_span("**whatever**"), Strong(vec![Text("whatever")]));

    assert_eq!(
        parse_span("__markdown is better than nothing__"),
        Strong(vec![Text("markdown is better than nothing")])
              );
    assert_eq!(
        parse_span("**markdown is better than nothing**"),
        Strong(vec![Text("markdown is better than nothing")])
              );

    assert_eq!(
        parse_span("__[an example](example.com) is better than nothing__"),
        Strong(vec![
                 Link("an example", "example.com", ""),
                 Text(" is better than nothing")
                 ])
              );

    assert_eq!(
        parse_span("**[an example](example.com) is better than nothing**"),
        Strong(vec![
                 Link("an example", "example.com", ""),
                 Text(" is better than nothing")
                 ])
              );

    assert_eq!(
        parse_span("**[an example](example.com) is __better__ than nothing**"),
        Strong(vec![
                 Link("an example", "example.com", ""),
                 Text(" is "),
                 Strong(vec![Text("better")]),
                 Text(" than nothing")
                 ])
              );

    assert_eq!(
        parse_span("**[an example](example.com) is *better* than nothing**"),
        Strong(vec![
                 Link("an example", "example.com", ""),
                 Text(" is "),
                 Emphasis(vec![Text("better")]),
                 Text(" than nothing")
                 ])
              );

    assert_eq!(
        parse_span("__[an example](example.com) is **better** than nothing__"),
        Strong(vec![
                 Link("an example", "example.com", ""),
                 Text(" is "),
                 Strong(vec![Text("better")]),
                 Text(" than nothing")
                 ])
              );

    assert_eq!(
        parse_span("__[an example](example.com) is _better_ than nothing__"),
        Strong(vec![
                 Link("an example", "example.com", ""),
                 Text(" is "),
                 Emphasis(vec![Text("better")]),
                 Text(" than nothing")
                 ])
              );

    // does not compile emphasis
    match parse_span("_whatever_"){
      Strong(_) => fail!(),
      _ => {}
    }
    match parse_span("*whatever*"){
      Strong(_) => fail!(),
      _ => {}
    }
}

