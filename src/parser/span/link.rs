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
        let mut content_end_index = 0;

        // This tracks open vs. closed braces, it starts at 1 because we have an initial
        // open brace, we want to reach 0 to find the closing brace for the link.
        assert_eq!('[', chars.next().unwrap());
        let mut brace_level = 1;

        // Walk through the link content matching braces to ensure that we find the correct closing
        // brace for the link, e.g. `[a link with ![an image](src) inside](link)` should not parse just
        // `[a link with ![an image]`.
        while let Some(next) = chars.next() {
            // Skip escaped braces.
            if next == '\\' {
                content_end_index += 1;
                if let Some(_) = chars.next() {
                    content_end_index += 1;
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
            content_end_index += 1;
        }
        let content = &text[1..content_end_index + 1];

        // Unmatched braces inside a link text are not supported on purpose, e.g. consider the case
        // of `The brace character ([) is parsed [like this](https://example.com)`. Here we want
        // to prefer the shorter link at the end instead of starting to parse at the first `[`.
        if brace_level != 0 {
            return None;
        }

        let caps = LINK_ATTR.captures(chars.as_str()).unwrap();
        let raw_content = &text[..content_end_index + 2 + caps[0].len()];

        // Check whether we have an inline link (in which case the "url" field is captured),
        // whether there's an explicit reference provided or if we should implicitly use the link
        // content as reference.
        if let Some(url) = caps.name("url") {
            let url = url.as_str().trim();
            let title = caps.name("title").map(|t| t.as_str());

            return Some((Link(parse_spans(&content), url, title), raw_content.len()));
        } else if let Some(reference) = caps.name("ref") {
            let reference = reference.as_str().trim().to_lowercase();

            return Some((RefLink(parse_spans(&content), reference, raw_content), raw_content.len()));
        } else {
            // Leave the reference empty, the HTML generating code will try to match both reference
            // and slugified content.
            let reference = "".to_owned();

            return Some((RefLink(parse_spans(&content), reference, raw_content), raw_content.len()));
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
                    vec![Text("an example")],
                    "example.com",
                    None
                ),
                25
            ))
        );

        assert_eq!(
            parse_link("[an example][example]"),
            Some((
                RefLink(
                    vec![Text("an example")],
                    "example".to_owned(),
                    "[an example][example]"
                ),
                21
            ))
        );

        assert_eq!(
            parse_link("[](example.com) test"),
            Some((Link(vec![], "example.com", None), 15))
        );

        assert_eq!(
            parse_link("[an example]() test"),
            Some((
                Link(vec![Text("an example")], "", None),
                14
            ))
        );

        assert_eq!(
            parse_link("[]() test"),
            Some((Link(vec![], "", None), 4))
        );

        assert_eq!(
            parse_link("[()] test"),
            Some((
                RefLink(
                    vec![Text("()")],
                    "".to_owned(),
                    "[()]"
                ),
                4
            ))
        );

        assert_eq!(
            parse_link("[an example](example.com \"Title\") test"),
            Some((
                Link(
                    vec![Text("an example")],
                    "example.com",
                    Some("Title")
                ),
                33
            ))
        );

        assert_eq!(
            parse_link("[an example](example.com) test [a link](example.com)"),
            Some((
                Link(
                    vec![Text("an example")],
                    "example.com",
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
                    vec![Image("test", "abc", None)],
                    "example.com",
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
                        Text("huh"),
                        RefLink(vec![], "".to_owned(), "[]"),
                        Text("wow")
                    ],
                    "example.com",
                    None
                ),
                23
            ))
        );

        assert_eq!(
            parse_link("[huh\\[wow](example.com)"),
            Some((
                Link(
                    vec![Text("huh"), Literal('['), Text("wow")],
                    "example.com",
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
                    vec![Text("an example")],
                    "example.com",
                    Some("Title (huh!)")
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
                    vec![Text("an example")],
                    "example".to_owned(),
                    "[an example]      [example]"
                ),
                27
            ))
        );

        assert_eq!(
            parse_link("[an example](example.com           \"Title\") test"),
            Some((
                Link(
                    vec![Text("an example")],
                    "example.com",
                    Some("Title")
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
