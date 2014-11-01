use regex::Regex;
use parser::{Break, Span, Text, Emphasis, Strong, Code, Link, Image};

static SPANS : Regex = regex!(r"(!?\[.*\]\([^\(\)]*\))|(\*[^\*].+?\*)|(\*\*.+?\*\*)|(_[^_].+?_)|(__.+?__)|(`[^`].+?`)|(``.+?``)|( {2})$");
static LINK  : Regex = regex!("\\[(?P<text>.*)\\]\\((?P<url>.*?)(?:\\s\"(?P<title>.*?)\")?\\)");
static IMAGE  : Regex = regex!("!\\[(?P<text>.*)\\]\\((?P<url>.*?)(?:\\s\"(?P<title>.*?)\")?\\)");
static EMPHASIS_UNDERSCORE  : Regex = regex!(r"^_(?P<text>[^_].+?)_");
static EMPHASIS_STAR  : Regex = regex!(r"^\*(?P<text>[^\*].+?)\*");
static STRONG_UNDERSCORE  : Regex = regex!(r"^__(?P<text>.+?)__");
static STRONG_STAR  : Regex = regex!(r"^\*\*(?P<text>.+?)\*\*");
static CODE_SINGLE  : Regex = regex!(r"^`(?P<text>[^`].+?)`");
static CODE_DOUBLE  : Regex = regex!(r"^``(?P<text>.+?)``");
static BREAK : Regex = regex!(r" {2}$");

pub fn parse_spans(text : &str) -> Vec<Span>{
    let mut tokens = vec![];
    let mut current = 0;
    for (begin, end) in SPANS.find_iter(text) {
        match text.slice(current, begin){
            "" => {}
            t => tokens.push(Text(t.to_string()))
        };
        tokens.push(parse_span(text.slice(begin, end)));
        current = end;
    }
    match text.slice(current, text.len()){
        "" => {}
        t => tokens.push(Text(t.to_string()))
    };
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
        return Code(caps.name("text").to_string());
    }else if CODE_SINGLE.is_match(text){
        let caps = CODE_SINGLE.captures(text).unwrap();
        return Code(caps.name("text").to_string());

    }else if IMAGE.is_match(text){
        let caps = IMAGE.captures(text).unwrap();
        return Image(
            caps.name("text").to_string(),
            caps.name("url").to_string(),
            caps.name("title").to_string()
            );

    }else if LINK.is_match(text){
        let caps = LINK.captures(text).unwrap();
        return Link(
            caps.name("text").to_string(),
            caps.name("url").to_string(),
            caps.name("title").to_string()
            );

    }else if BREAK.is_match(text){
        return Break;
    }

    return Text(text.to_string());
}

// HERE BE TEST

#[test]
fn parse_break_test() {
    assert_eq!(parse_span("  "), Break);
    assert_eq!(parse_spans("this is a test  "), vec![Text("this is a test".to_string()), Break])
    match parse_span(" "){
      Break => panic!(),
      _ => {}
    }
}


#[test]
fn parse_link_test() {
    assert_eq!(parse_span("[an example](example.com)"),
                Link("an example".to_string(), "example.com".to_string(), "".to_string()));

    assert_eq!(parse_span("[an example](example.com \"Title\")"),
                Link("an example".to_string(), "example.com".to_string(), "Title".to_string()));
}

#[test]
fn parse_image_test() {
    assert_eq!(parse_span("![an example](example.com)"),
                Image("an example".to_string(), "example.com".to_string(), "".to_string()));
    assert_eq!(parse_span("![an example](example.com \"Title\")"),
                Image("an example".to_string(), "example.com".to_string(), "Title".to_string()));
}


#[test]
fn parse_emphasis_test() {

    assert_eq!(parse_span("_whatever_"), Emphasis(vec![Text("whatever".to_string())]));
    assert_eq!(parse_span("*whatever*"), Emphasis(vec![Text("whatever".to_string())]));

    assert_eq!(
        parse_span("_markdown is better than nothing_"),
        Emphasis(vec![Text("markdown is better than nothing".to_string())])
              );
    assert_eq!(
        parse_span("*markdown is better than nothing*"),
        Emphasis(vec![Text("markdown is better than nothing".to_string())])
              );

    assert_eq!(
        parse_span("_[an example](example.com) is better than nothing_"),
        Emphasis(vec![
                 Link("an example".to_string(), "example.com".to_string(), "".to_string()),
                 Text(" is better than nothing".to_string())
                 ])
              );

    assert_eq!(
        parse_span("*[an example](example.com) is better than nothing*"),
        Emphasis(vec![
                 Link("an example".to_string(), "example.com".to_string(), "".to_string()),
                 Text(" is better than nothing".to_string())
                 ])
              );

    assert_eq!(
        parse_span("*[an example](example.com) is _better_ than nothing*"),
        Emphasis(vec![
                 Link("an example".to_string(), "example.com".to_string(), "".to_string()),
                 Text(" is ".to_string()),
                 Emphasis(vec![Text("better".to_string())]),
                 Text(" than nothing".to_string())
                 ])
              );

    assert_eq!(
        parse_span("_[an example](example.com) is *better* than nothing_"),
        Emphasis(vec![
                 Link("an example".to_string(), "example.com".to_string(), "".to_string()),
                 Text(" is ".to_string()),
                 Emphasis(vec![Text("better".to_string())]),
                 Text(" than nothing".to_string())
                 ])
              );

    // does not compile strong
    match parse_span("__whatever__"){
      Emphasis(_) => panic!(),
      _ => {}
    }
    match parse_span("**whatever**"){
      Emphasis(_) => panic!(),
      _ => {}
    }
}

#[test]
fn parse_strong_test() {

    assert_eq!(parse_span("__whatever__"), Strong(vec![Text("whatever".to_string())]));
    assert_eq!(parse_span("**whatever**"), Strong(vec![Text("whatever".to_string())]));

    assert_eq!(
        parse_span("__markdown is better than nothing__"),
        Strong(vec![Text("markdown is better than nothing".to_string())])
              );
    assert_eq!(
        parse_span("**markdown is better than nothing**"),
        Strong(vec![Text("markdown is better than nothing".to_string())])
              );

    assert_eq!(
        parse_span("__[an example](example.com) is better than nothing__"),
        Strong(vec![
                 Link("an example".to_string(), "example.com".to_string(), "".to_string()),
                 Text(" is better than nothing".to_string())
                 ])
              );

    assert_eq!(
        parse_span("**[an example](example.com) is better than nothing**"),
        Strong(vec![
                 Link("an example".to_string(), "example.com".to_string(), "".to_string()),
                 Text(" is better than nothing".to_string())
                 ])
              );

    assert_eq!(
        parse_span("**[an example](example.com) is __better__ than nothing**"),
        Strong(vec![
                 Link("an example".to_string(), "example.com".to_string(), "".to_string()),
                 Text(" is ".to_string()),
                 Strong(vec![Text("better".to_string())]),
                 Text(" than nothing".to_string())
                 ])
              );

    assert_eq!(
        parse_span("**[an example](example.com) is *better* than nothing**"),
        Strong(vec![
                 Link("an example".to_string(), "example.com".to_string(), "".to_string()),
                 Text(" is ".to_string()),
                 Emphasis(vec![Text("better".to_string())]),
                 Text(" than nothing".to_string())
                 ])
              );

    assert_eq!(
        parse_span("__[an example](example.com) is **better** than nothing__"),
        Strong(vec![
                 Link("an example".to_string(), "example.com".to_string(), "".to_string()),
                 Text(" is ".to_string()),
                 Strong(vec![Text("better".to_string())]),
                 Text(" than nothing".to_string())
                 ])
              );

    assert_eq!(
        parse_span("__[an example](example.com) is _better_ than nothing__"),
        Strong(vec![
                 Link("an example".to_string(), "example.com".to_string(), "".to_string()),
                 Text(" is ".to_string()),
                 Emphasis(vec![Text("better".to_string())]),
                 Text(" than nothing".to_string())
                 ])
              );

    // does not compile emphasis
    match parse_span("_whatever_"){
      Strong(_) => panic!(),
      _ => {}
    }
    match parse_span("*whatever*"){
      Strong(_) => panic!(),
      _ => {}
    }
}

