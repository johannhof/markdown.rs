use parser::span::parse_spans;
use parser::Span;
use parser::Span::{Link, RefLink};
use regex::Regex;

pub fn parse_link(text: &str) -> Option<(Span, usize)> {
    lazy_static! {
        // This is the second part of the regex, that matches the reference or url and title.
        static ref LINK_ATTR_STR: &'static str = "(?:\\s*\\[(?P<ref>.*)\\]|\\((?P<url>.*?)(?:\\s*\"(?P<title>.*?)\")?\\s*\\))?";
        // This regex does not sufficiently cover the edge case where there are brackets (e.g. for
        // images) inside a link text. It's sufficient for identifying links anyway, we'll properly
        // figure out the braces below.
        static ref LINK_STR: String = "^\\[(?P<text>.*?)\\]".to_owned() + &LINK_ATTR_STR;
        static ref LINK: Regex =
            Regex::new(&LINK_STR).unwrap();
        static ref LINK_ATTR: Regex =
            Regex::new(&("^".to_owned() + &LINK_ATTR_STR)).unwrap();
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

        // Check whether we have an inline link (in which case the "url" field is captured),
        // whether there's an explicit reference provided or if we should implicitly use the link
        // content as reference.
        if let Some(url) = caps.name("url") {
            let url = url.as_str().trim().to_owned();
            let title = caps.name("title").map(|t| t.as_str().to_owned());
            let len = 1 + content.len() + 1 + caps[0].len();

            return Some((Link(parse_spans(&content), url, title), len));
        } else if let Some(reference) = caps.name("ref") {
            let reference = reference.as_str().trim().to_lowercase();
            let len = 1 + content.len() + 1 + caps[0].len();
            let raw = ["[", &content, "]", &caps[0]].join("");

            return Some((RefLink(parse_spans(&content), reference, raw), len));
        } else {
            // Leave the reference empty, the HTML generating code will try to match both reference
            // and slugified content.
            let reference = "".to_owned();
            let len = 1 + content.len() + 1;
            let raw = ["[", &content, "]"].join("");

            return Some((RefLink(parse_spans(&content), reference, raw), len));
        }
    }
    None
}

#[cfg(test)]
mod test {
    use parser::span::parse_link;
    use parser::Span::{Image, Link, Literal, RefLink, Text};

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
            parse_link("[an example][example]"),
            Some((
                RefLink(
                    vec![Text("an example".to_owned())],
                    "example".to_owned(),
                    "[an example][example]".to_owned()
                ),
                21
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
            parse_link("[()] test"),
            Some((
                RefLink(
                    vec![Text("()".to_owned())],
                    "".to_owned(),
                    "[()]".to_owned()
                ),
                4
            ))
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
                    vec![
                        Text("huh".to_owned()),
                        RefLink(vec![], "".to_owned(), "[]".to_owned()),
                        Text("wow".to_owned())
                    ],
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
    fn space_length() {
        assert_eq!(
            parse_link("[an example]      [example]"),
            Some((
                RefLink(
                    vec![Text("an example".to_owned())],
                    "example".to_owned(),
                    "[an example]      [example]".to_owned()
                ),
                27
            ))
        );

        assert_eq!(
            parse_link("[an example](example.com           \"Title\") test"),
            Some((
                Link(
                    vec![Text("an example".to_owned())],
                    "example.com".to_owned(),
                    Some("Title".to_owned())
                ),
                43
            ))
        );
    }

    #[test]
    fn no_false_positives() {
        assert_eq!(parse_link("()[] testing things test"), None);
        assert_eq!(parse_link("[[][[]] testing things test"), None);
    }

    #[test]
    fn no_early_matching() {
        assert_eq!(parse_link("were [an example](example.com) test"), None);
    }
}
