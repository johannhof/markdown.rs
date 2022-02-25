use parser::Span;
use parser::Span::Image;
use regex::Regex;

pub fn parse_image(text: &str) -> Option<(Span, usize)> {
    lazy_static! {
        static ref IMAGE: Regex =
            Regex::new("^!\\[(?P<text>.*?)\\]\\((?P<url>.*?)(?:\\s\"(?P<title>.*?)\")?\\)")
                .unwrap();
    }

    if IMAGE.is_match(text) {
        let caps = IMAGE.captures(text).unwrap();
        let text = if let Some(mat) = caps.name("text") {
            mat.as_str()
        } else {
            ""
        };
        let url = if let Some(mat) = caps.name("url") {
            mat.as_str()
        } else {
            ""
        };
        let title = if let Some(mat) = caps.name("title") {
            Some(mat.as_str())
        } else {
            None
        };
        // TODO correctly get whitespace length between url and title
        let len = text.len() + url.len() + 5 + title.clone().map_or(0, |t| t.len() + 3);
        return Some((Image(text, url, title), len));
    }
    None
}

#[test]
fn finds_image() {
    assert_eq!(
        parse_image("![an example](example.com) test"),
        Some((
            Image("an example", "example.com", None),
            26
        ))
    );

    assert_eq!(
        parse_image("![](example.com) test"),
        Some((Image("", "example.com", None), 16))
    );

    assert_eq!(
        parse_image("![an example]() test"),
        Some((Image("an example", "", None), 15))
    );

    assert_eq!(
        parse_image("![]() test"),
        Some((Image("", "", None), 5))
    );

    assert_eq!(
        parse_image("![an example](example.com \"Title\") test"),
        Some((
            Image(
                "an example",
                "example.com",
                Some("Title")
            ),
            34
        ))
    );

    assert_eq!(
        parse_image("![an example](example.com) test [a link](example.com)"),
        Some((
            Image("an example", "example.com", None),
            26
        ))
    );
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
