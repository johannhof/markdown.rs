use parser::Block;
use parser::Block::LinkReference;
use regex::Regex;

pub fn parse_link_reference(lines: &[&str]) -> Option<(Block, usize)> {
    lazy_static! {
        static ref LINK_REFERENCE: Regex =
            Regex::new("^\\[(?P<reference>.*?)\\]:\\s(?P<url>.*?)(?:\\s\"(?P<title>.*?)\")?$").unwrap();
    }

    if LINK_REFERENCE.is_match(lines[0]) {
        let caps = LINK_REFERENCE.captures(lines[0]).unwrap();
        let text = if let Some(mat) = caps.name("reference") {
            if mat.as_str().is_empty() {
                //Don't parse a link ref without a ref
                return None;
            }

            mat.as_str().to_owned()
        } else {
            //Don't parse a link reference without a reference
            return None;
        };

        let url = if let Some(mat) = caps.name("url") {
            if mat.as_str().is_empty() {
                //Don't parse a link ref without a link
                return None;
            }

            mat.as_str().to_owned()
        } else {
            //Don't parse a link reference without a link
            return None;
        };

        let title = if let Some(mat) = caps.name("title") {
            Some(mat.as_str().to_owned())
        } else {
            //Titles are optional
            None
        };

        return Some((LinkReference(text, url, title), 1));
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
            parse_link_reference(&vec!["[ref]: link \"title\"", "next line"]).unwrap(),
            (LinkReference("ref".to_owned(), "link".to_owned(), Some("title".to_owned())), 1)
        );

        assert_eq!(
            parse_link_reference(&vec!["[ref]: link", "next line"]).unwrap(),
            (LinkReference("ref".to_owned(), "link".to_owned(), None), 1)
        );
    }

    #[test]
    fn no_false_positives() {
        assert_eq!(
            parse_link_reference(&vec!["[]: link \"title\"", "next line"]),
            None
        );

        assert_eq!(
            parse_link_reference(&vec!["[ref]: ", "next line"]),
            None
        );

        assert_eq!(
            parse_link_reference(&vec!["ahh [ref]: link \"title\"", "next line"]),
            None
        );
    }
}
