use regex::Regex;
use parser::Span;
use parser::Span::Link;

pub fn parse_link(text: &str) -> Option<(Span, usize)>{
    let LINK = Regex::new("^\\[(?P<text>.*?)\\]\\((?P<url>.*?)(?:\\s\"(?P<title>.*?)\")?\\)").unwrap();

    if LINK.is_match(text){
        let caps = LINK.captures(text).unwrap();
        let text = caps.name("text").unwrap_or("").to_string();
        let url = caps.name("url").unwrap_or("").to_string();
        let title = caps.name("title").map(|t| t.to_string());
        // TODO correctly get whitespace length between url and title
        let len = text.len() + url.len() + 4 + title.clone().map_or(0, |t| t.len() + 3);
        return Some((Link(text, url, title), len));
    }
    return None;
}

#[test]
fn finds_link() {
    assert_eq!(
        parse_link("[an example](example.com) test"),
        Some((Link("an example".to_string(), "example.com".to_string(), None), 25))
    );

    assert_eq!(
        parse_link("[](example.com) test"),
        Some((Link("".to_string(), "example.com".to_string(), None), 15))
    );

    assert_eq!(
        parse_link("[an example]() test"),
        Some((Link("an example".to_string(), "".to_string(), None), 14))
    );

    assert_eq!(
        parse_link("[]() test"),
        Some((Link("".to_string(), "".to_string(), None), 4))
    );

    assert_eq!(
        parse_link("[an example](example.com \"Title\") test"),
        Some((Link("an example".to_string(), "example.com".to_string(), Some("Title".to_string())), 33))
    );

    assert_eq!(
        parse_link("[an example](example.com) test [a link](example.com)"),
        Some((Link("an example".to_string(), "example.com".to_string(), None), 25))
    );
}

#[test]
fn no_false_positives() {
    assert_eq!(
        parse_link("[()] testing things test"),
        None
    );
    assert_eq!(
        parse_link("()[] testing things test"),
        None
    );
}

#[test]
fn no_early_matching() {
    assert_eq!(
        parse_link("were [an example](example.com) test"),
        None
    );
}

