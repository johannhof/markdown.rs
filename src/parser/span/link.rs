use parser::span::parse_spans;
use parser::Span;
use parser::Span::Link;
use regex::Regex;

pub fn parse_link(text: &str) -> Option<(Span, usize)> {
    lazy_static! {
        // This regex does not sufficiently cover the edge case where there are brackets (e.g. for
        // images) inside a link text. It's sufficient for identifying links anyway, we'll properly
        // figure out the braces below.
        static ref LINK: Regex =
            Regex::new("^\\[(?P<text>.*?)\\]\\((?P<url>.*?)(?:\\s\"(?P<title>.*?)\")?\\)").unwrap();
        // This is simply the second part of the above regex, that matches the url and title.
        static ref LINK_ATTR: Regex =
            Regex::new("^\\((?P<url>.*?)(?:\\s\"(?P<title>.*?)\")?\\)").unwrap();
    }

    if LINK.is_match(text) {
        let mut chars = text.chars();
        let mut content = String::new();

        // This tracks open vs. closed braces, it starts at 1 because we have an initial
        // open brace, we want to reach 0 to find the closing brace for the link.
        assert_eq!('[', chars.next().unwrap());
        let mut brace_level = 1;

        // Walk through the link content matching braces to ensure that we find the correct closing
        // brace for the link, e.g. `[a link with ![an image](src) inside](link)` should not only parse
        // `[a link with ![an image]`.
        while let Some(next) = chars.next() {
            // Skip escaped braces.
            if next == '\\' {
                content.push(next);
                if let Some(x) = chars.next() {
                    content.push(x);
                }
                continue;
            } else if next == ']' {
                brace_level -= 1;
            } else if next == '[' {
                brace_level += 1;
            }
            if brace_level == 0 {
                break;
            }
            content.push(next);
        }

        // Unmatched braces inside a link text are not supported on purpose, e.g. consider the case
        // of `The brace character ([) is parsed [like this](https://example.com)`. Here we want
        // to prefer the shorter link at the end instead of starting to parse at the first `[`.
        if brace_level != 0 {
            return None;
        }

        let caps = LINK_ATTR.captures(chars.as_str()).unwrap();
        let url = caps.name("url").map_or("", |t| t.as_str()).to_owned();
        let title = caps.name("title").map(|t| t.as_str().to_owned());

        // TODO correctly get whitespace length between url and title
        let len = content.len() + url.len() + 4 + title.as_ref().map_or(0, |t| t.len() + 3);
        return Some((Link(parse_spans(&content), url, title), len));
    }
    None
}

#[cfg(test)]
mod test {
    use parser::span::parse_link;
    use parser::Span::{Image, Link, Literal, Text};

    #[test]
    fn finds_link() {
        assert_eq!(
            parse_link("[an example](example.com) test"),
            Some((
                Link(
                    vec![Text("an example".to_owned())],
                    "example.com".to_owned(),
                    None
                ),
                25
            ))
        );

        assert_eq!(
            parse_link("[](example.com) test"),
            Some((Link(vec![], "example.com".to_owned(), None), 15))
        );

        assert_eq!(
            parse_link("[an example]() test"),
            Some((
                Link(vec![Text("an example".to_owned())], "".to_owned(), None),
                14
            ))
        );

        assert_eq!(
            parse_link("[]() test"),
            Some((Link(vec![], "".to_owned(), None), 4))
        );

        assert_eq!(
            parse_link("[an example](example.com \"Title\") test"),
            Some((
                Link(
                    vec![Text("an example".to_owned())],
                    "example.com".to_owned(),
                    Some("Title".to_owned())
                ),
                33
            ))
        );

        assert_eq!(
            parse_link("[an example](example.com) test [a link](example.com)"),
            Some((
                Link(
                    vec![Text("an example".to_owned())],
                    "example.com".to_owned(),
                    None
                ),
                25
            ))
        );
    }

    #[test]
    fn brackets_in_link() {
        assert_eq!(
            parse_link("[![test](abc)](example.com) test [a link](example.com)"),
            Some((
                Link(
                    vec![Image("test".to_owned(), "abc".to_owned(), None)],
                    "example.com".to_owned(),
                    None
                ),
                27
            ))
        );

        assert_eq!(
            parse_link("[huh[]wow](example.com)"),
            Some((
                Link(
                    vec![Text("huh[]wow".to_owned())],
                    "example.com".to_owned(),
                    None
                ),
                23
            ))
        );

        assert_eq!(
            parse_link("[huh\\[wow](example.com)"),
            Some((
                Link(
                    vec![Text("huh".to_owned()), Literal('['), Text("wow".to_owned())],
                    "example.com".to_owned(),
                    None
                ),
                23
            ))
        );

        assert_eq!(parse_link("[huh[wow](example.com)"), None);

        assert_eq!(
            parse_link("[an example](example.com \"Title (huh!)\") test"),
            Some((
                Link(
                    vec![Text("an example".to_owned())],
                    "example.com".to_owned(),
                    Some("Title (huh!)".to_owned())
                ),
                40
            ))
        );
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
