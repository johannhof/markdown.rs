use parser::span::parse_spans;
use parser::Block;
use parser::Block::Header;
use regex::Regex;

pub fn parse_atx_header<'a>(lines: &[&'a str]) -> Option<(Block<'a>, usize)> {
    lazy_static! {
        static ref ATX_HEADER_RE: Regex =
            Regex::new(r"^(?P<level>#{1,6})\s(?P<text>.*?)(?:\s#*)?$").unwrap();
    }

    if ATX_HEADER_RE.is_match(lines[0]) {
        let caps = ATX_HEADER_RE.captures(lines[0]).unwrap();
        return Some((
            Header(
                parse_spans(caps.name("text").unwrap().as_str()),
                caps.name("level").unwrap().as_str().len(),
            ),
            1,
        ));
    }
    None
}

#[cfg(test)]
mod test {
    use super::parse_atx_header;
    use parser::Block::Header;
    use parser::Span::Text;

    #[test]
    fn finds_atx_header() {
        assert_eq!(
            parse_atx_header(&vec!["### Test", "testtest"]).unwrap(),
            (Header(vec![Text("Test".into())], 3), 1)
        );

        assert_eq!(
            parse_atx_header(&vec!["# Test", "testtest"]).unwrap(),
            (Header(vec![Text("Test".into())], 1), 1)
        );

        assert_eq!(
            parse_atx_header(&vec!["###### Test", "testtest"]).unwrap(),
            (Header(vec![Text("Test".into())], 6), 1)
        );

        assert_eq!(
            parse_atx_header(&vec!["### Test and a pretty long sentence", "testtest"]).unwrap(),
            (
                Header(vec![Text("Test and a pretty long sentence".into())], 3),
                1
            )
        );
    }

    #[test]
    fn ignores_closing_hashes() {
        assert_eq!(
            parse_atx_header(&vec!["### Test ###", "testtest"]).unwrap(),
            (Header(vec![Text("Test".into())], 3), 1)
        );

        assert_eq!(
            parse_atx_header(&vec!["# Test #", "testtest"]).unwrap(),
            (Header(vec![Text("Test".into())], 1), 1)
        );

        assert_eq!(
            parse_atx_header(&vec!["###### Test ##", "testtest"]).unwrap(),
            (Header(vec![Text("Test".into())], 6), 1)
        );

        assert_eq!(
            parse_atx_header(&vec![
                "### Test and a pretty long sentence #########",
                "testtest"
            ])
            .unwrap(),
            (
                Header(vec![Text("Test and a pretty long sentence".into())], 3),
                1
            )
        );
    }

    #[test]
    fn no_false_positives() {
        assert_eq!(parse_atx_header(&vec!["####### Test", "testtest"]), None);
        assert_eq!(parse_atx_header(&vec!["Test #", "testtest"]), None);
        assert_eq!(parse_atx_header(&vec!["T ### est #", "testtest"]), None);
    }
}
