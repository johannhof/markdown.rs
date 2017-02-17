use regex::Regex;
use parser::Span;
use parser::Span::Image;

pub fn parse_image(text: &str) -> Option<(Span, usize)> {
    lazy_static! {
        static ref IMAGE :Regex = Regex::new("^!\\[(?P<text>.*?)\\]\\((?P<url>.*?)(?:\\s\"(?P<title>.*?)\")?\\)").unwrap();
    }

    if IMAGE.is_match(text) {
        let caps = IMAGE.captures(text).unwrap();
        let text = caps.name("text").unwrap_or("").to_owned();
        let url = caps.name("url").unwrap_or("").to_owned();
        let title = caps.name("title").map(|t| t.to_owned());
        // TODO correctly get whitespace length between url and title
        let len = text.len() + url.len() + 5 + title.clone().map_or(0, |t| t.len() + 3);
        return Some((Image(text, url, title), len));
    }
    None
}

#[test]
fn finds_image() {
    assert_eq!(parse_image("![an example](example.com) test"),
               Some((Image("an example".to_owned(), "example.com".to_owned(), None),
                     26)));

    assert_eq!(parse_image("![](example.com) test"),
               Some((Image("".to_owned(), "example.com".to_owned(), None), 16)));

    assert_eq!(parse_image("![an example]() test"),
               Some((Image("an example".to_owned(), "".to_owned(), None), 15)));

    assert_eq!(parse_image("![]() test"),
               Some((Image("".to_owned(), "".to_owned(), None), 5)));

    assert_eq!(parse_image("![an example](example.com \"Title\") test"),
               Some((Image("an example".to_owned(),
                           "example.com".to_owned(),
                           Some("Title".to_owned())),
                     34)));

    assert_eq!(parse_image("![an example](example.com) test [a link](example.com)"),
               Some((Image("an example".to_owned(), "example.com".to_owned(), None),
                     26)));
}

#[test]
fn no_false_positives() {
    assert_eq!(parse_image("![()] testing things test"), None);
    assert_eq!(parse_image("!()[] testing things test"), None);
}

#[test]
fn no_early_matching() {
    assert_eq!(parse_image("were ![an example](example.com) test"), None);
}
