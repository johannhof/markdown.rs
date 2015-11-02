use regex::Regex;
use parser::span::parse_spans;
use parser::Span;
use parser::Span::Emphasis;

pub fn parse_emphasis(text: &str) -> Option<(Span, usize)> {
    let emphasis_underscore = Regex::new(r"^_(?P<text>.+?)_").unwrap();
    let emphasis_star = Regex::new(r"^\*(?P<text>.+?)\*").unwrap();

    if emphasis_underscore.is_match(text) {
        let caps = emphasis_underscore.captures(text).unwrap();
        let t = caps.name("text").unwrap();
        return Some((Emphasis(parse_spans(t)), t.len() + 2));
    } else if emphasis_star.is_match(text) {
        let caps = emphasis_star.captures(text).unwrap();
        let t = caps.name("text").unwrap();
        return Some((Emphasis(parse_spans(t)), t.len() + 2));
    }
    None
}

#[cfg(test)]
mod test {
    use parser::Span::{Text, Emphasis};
    use super::parse_emphasis;

    #[test]
    fn finds_emphasis() {
        assert_eq!(parse_emphasis("_testing things_ test"),
                   Some((Emphasis(vec![Text("testing things".to_owned())]), 16)));

        assert_eq!(parse_emphasis("*testing things* test"),
                   Some((Emphasis(vec![Text("testing things".to_owned())]), 16)));

        assert_eq!(parse_emphasis("_testing things_ things_ test"),
                   Some((Emphasis(vec![Text("testing things".to_owned())]), 16)));

        assert_eq!(parse_emphasis("_w_ things_ test"),
                   Some((Emphasis(vec![Text("w".to_owned())]), 3)));

        assert_eq!(parse_emphasis("*w* things* test"),
                   Some((Emphasis(vec![Text("w".to_owned())]), 3)));

        assert_eq!(parse_emphasis("_w__ testing things test"),
                   Some((Emphasis(vec![Text("w".to_owned())]), 3)));
    }

    #[test]
    fn no_false_positives() {
        assert_eq!(parse_emphasis("__ testing things test"), None);
        assert_eq!(parse_emphasis("_ test"), None);
    }

    #[test]
    fn no_early_matching() {
        assert_eq!(parse_emphasis("were _testing things_ test"), None);
        assert_eq!(parse_emphasis("were *testing things* test"), None);
    }
}
