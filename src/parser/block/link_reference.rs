use parser::Block;
use parser::Block::LinkReference;
use regex::Regex;

pub fn parse_link_reference<'a>(lines: &[&'a str]) -> Option<(Block<'a>, usize)> {
    lazy_static! {
        static ref LINK_REFERENCE_SINGLE_LINE: Regex = Regex::new("^\\s*\\[(?P<id>[^\\[\\]]+)\\]:\\s*(?P<url>\\S+)(?:\\s+(?:'(?P<title1>.*)'|\"(?P<title2>.*)\"|\\((?P<title3>.*?)\\)))?\n?").unwrap();
        static ref LINK_REFERENCE_FIRST_LINE: Regex = Regex::new("^\\s*\\[(?P<id>[^\\[\\]]+)\\]:").unwrap();
        static ref LINK_REFERENCE_SECOND_LINE: Regex = Regex::new("\\s*(?P<url>\\S+)(?:\\s+(?:'(?P<title1>.*)'|\"(?P<title2>.*)\"|\\((?P<title3>.*?)\\)))?\n?").unwrap();
    }

    if LINK_REFERENCE_SINGLE_LINE.is_match(lines[0]) {
        let caps = LINK_REFERENCE_SINGLE_LINE.captures(lines[0]).unwrap();
        return Some((
            LinkReference(
                caps.name("id").unwrap().as_str().to_lowercase(),
                caps.name("url").unwrap().as_str(),
                caps.name("title1")
                    .or_else(|| caps.name("title2"))
                    .or_else(|| caps.name("title3"))
                    .map(|s| s.as_str()),
            ),
            1,
        ));
    }

    if LINK_REFERENCE_FIRST_LINE.is_match(lines[0]) && LINK_REFERENCE_SECOND_LINE.is_match(lines[1])
    {
        let caps1 = LINK_REFERENCE_FIRST_LINE.captures(lines[0]).unwrap();
        let caps2 = LINK_REFERENCE_SECOND_LINE.captures(lines[1]).unwrap();
        return Some((
            LinkReference(
                caps1.name("id").unwrap().as_str().to_lowercase(),
                caps2.name("url").unwrap().as_str(),
                caps2
                    .name("title1")
                    .or_else(|| caps2.name("title2"))
                    .or_else(|| caps2.name("title3"))
                    .map(|s| s.as_str()),
            ),
            2,
        ));
    }

    None
}

#[cfg(test)]
mod test {
    use super::parse_link_reference;
    use parser::Block::LinkReference;

    #[test]
    fn finds_link_reference() {
        assert_eq!(
            parse_link_reference(&vec!["[Test]: https://example.com"]).unwrap(),
            (
                LinkReference("test".to_owned(), "https://example.com", None),
                1
            )
        );

        assert_eq!(
            parse_link_reference(&vec!["[Test]: https://example.com \"example\""]).unwrap(),
            (
                LinkReference(
                    "test".to_owned(),
                    "https://example.com",
                    Some("example")
                ),
                1
            )
        );

        assert_eq!(
            parse_link_reference(&vec!["[Test]: https://example.com (example)"]).unwrap(),
            (
                LinkReference(
                    "test".to_owned(),
                    "https://example.com",
                    Some("example")
                ),
                1
            )
        );

        assert_eq!(
            parse_link_reference(&vec!["[Test]: https://example.com 'example'"]).unwrap(),
            (
                LinkReference(
                    "test".to_owned(),
                    "https://example.com",
                    Some("example")
                ),
                1
            )
        );

        assert_eq!(
            parse_link_reference(&vec!["[Test]:     https://example.com        'example'"])
                .unwrap(),
            (
                LinkReference(
                    "test".to_owned(),
                    "https://example.com",
                    Some("example")
                ),
                1
            )
        );

        assert_eq!(
            parse_link_reference(&vec!["[Test]:", "https://example.com \"example\""]).unwrap(),
            (
                LinkReference(
                    "test".to_owned(),
                    "https://example.com",
                    Some("example")
                ),
                2
            )
        );
    }
}
