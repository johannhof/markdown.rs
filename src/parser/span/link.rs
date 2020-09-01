use regex::Regex;
use parser::Span;
use parser::Span::Link;
use parser::span::parse_spans;

pub fn parse_link(text: &str) -> Option<(Span, usize)> {
    lazy_static! {
        static ref LINK :Regex = Regex::new("^\\[(?P<text>.*?)\\]\\((?P<url>.*?)(?:\\s\"(?P<title>.*?)\")?\\)").unwrap();
    }

    if LINK.is_match(text) {
        let caps = LINK.captures(text).unwrap();
        let text = if let Some(mat) = caps.name("text") { mat.as_str() } else { "" };
        let url = if let Some(mat) = caps.name("url") { mat.as_str().to_owned() } else { "".to_owned() };
        let title = if let Some(mat) = caps.name("title") { Some(mat.as_str().to_owned()) } else { None };
        // let title = caps.name("title").map(|t| t.to_owned());
        // TODO correctly get whitespace length between url and title
        let len = text.len() + url.len() + 4 + title.clone().map_or(0, |t| t.len() + 3);
        return Some((Link(parse_spans(text), url, title), len));
    }
    None
}

#[cfg(test)]
mod test {
    use parser::Span::{Text, Break, Code, Emphasis, Strong, Link, Image};
    use parser::span::parse_link;

    #[test]
    fn finds_link() {
        assert_eq!(parse_link("[an example](example.com) test"),
                   Some((Link(vec![Text("an example".to_owned())], "example.com".to_owned(), None),
                         25)));

        assert_eq!(parse_link("[](example.com) test"),
                   Some((Link(vec![], "example.com".to_owned(), None), 15)));

        assert_eq!(parse_link("[an example]() test"),
                   Some((Link(vec![Text("an example".to_owned())], "".to_owned(), None), 14)));

        assert_eq!(parse_link("[]() test"),
                   Some((Link(vec![], "".to_owned(), None), 4)));

        assert_eq!(parse_link("[an example](example.com \"Title\") test"),
                   Some((Link(vec![Text("an example".to_owned())],
                              "example.com".to_owned(),
                              Some("Title".to_owned())),
                         33)));

        assert_eq!(parse_link("[an example](example.com) test [a link](example.com)"),
                   Some((Link(vec![Text("an example".to_owned())], "example.com".to_owned(), None),
                         25)));
    }

    #[test]
    fn no_false_positives() {
        assert_eq!(parse_link("[()] testing things test"), None);
        assert_eq!(parse_link("()[] testing things test"), None);
    }

    #[test]
    fn no_early_matching() {
        assert_eq!(parse_link("were [an example](example.com) test"), None);
    }
}
