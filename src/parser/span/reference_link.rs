use parser::Span;
use parser::Span::ReferenceLink;
use regex::Regex;

pub fn parse_reference_link(text: &str) -> Option<(Span, usize)> {
    lazy_static! {
        static ref LINK: Regex = Regex::new("^\\[(?P<text>.*?)\\]\\[(?P<url>.*?)\\]").unwrap();
    }

    if LINK.is_match(text) {
        let caps = LINK.captures(text).unwrap();
        let text = if let Some(mat) = caps.name("text") {
            mat.as_str().to_owned()
        } else {
            "".to_owned()
        };
        let url = if let Some(mat) = caps.name("url") {
            mat.as_str().to_owned()
        } else {
            "".to_owned()
        };

        let len = text.len() + url.len() + 4;
        return Some((ReferenceLink(text, url), len));
    }
    None
}

#[test]
fn finds_link() {
    assert_eq!(
        parse_reference_link("[an example][ref] test"),
        Some((ReferenceLink("an example".to_owned(), "ref".to_owned()), 17))
    );

    assert_eq!(
        parse_reference_link("[][ref] test"),
        Some((ReferenceLink("".to_owned(), "ref".to_owned()), 7))
    );

    assert_eq!(
        parse_reference_link("[an example][] test"),
        Some((ReferenceLink("an example".to_owned(), "".to_owned()), 14))
    );

    assert_eq!(
        parse_reference_link("[][] test"),
        Some((ReferenceLink("".to_owned(), "".to_owned()), 4))
    );

    assert_eq!(
        parse_reference_link("[an example][example] test [a link](example.com)"),
        Some((
            ReferenceLink("an example".to_owned(), "example".to_owned()),
            21
        ))
    );
}

#[test]
fn no_false_positives() {
    assert_eq!(parse_reference_link("[[]] testing things test"), None);
    assert_eq!(parse_reference_link("[[]] testing things test"), None);
    assert_eq!(parse_reference_link("[]: testing things test"), None);
    assert_eq!(parse_reference_link("][[] testing things test"), None);
}

#[test]
fn no_early_matching() {
    assert_eq!(
        parse_reference_link("were [an example][example] test"),
        None
    );
}
