use regex::Regex;
use parser::span::parse_spans;
use parser::Span;
use parser::Span::Emphasis;

pub fn parse_emphasis(text: &str) -> Option<(Span, usize)>{
    let EMPHASIS_UNDERSCORE = Regex::new(r"^_(?P<text>.+?)_").unwrap();
    let EMPHASIS_STAR = Regex::new(r"^\*(?P<text>.+?)\*").unwrap();

    if EMPHASIS_UNDERSCORE.is_match(text){
        let caps = EMPHASIS_UNDERSCORE.captures(text).unwrap();
        let t = caps.name("text").unwrap();
        return Some((Emphasis(parse_spans(t)), t.len() + 2));
    }else if EMPHASIS_STAR.is_match(text){
        let caps = EMPHASIS_STAR.captures(text).unwrap();
        let t = caps.name("text").unwrap();
        return Some((Emphasis(parse_spans(t)), t.len() + 2));
    }
    return None;
}

#[cfg(test)]
mod test {
    use parser::Span::{Text, Emphasis};
    use super::parse_emphasis;

    #[test]
    fn finds_emphasis() {
        assert_eq!(
            parse_emphasis("_testing things_ test"),
            Some((Emphasis(vec![Text("testing things".to_string())]), 16))
        );

        assert_eq!(
            parse_emphasis("*testing things* test"),
            Some((Emphasis(vec![Text("testing things".to_string())]), 16))
        );

        assert_eq!(
            parse_emphasis("_testing things_ things_ test"),
            Some((Emphasis(vec![Text("testing things".to_string())]), 16))
        );

        assert_eq!(
            parse_emphasis("_w_ things_ test"),
            Some((Emphasis(vec![Text("w".to_string())]), 3))
        );

        assert_eq!(
            parse_emphasis("*w* things* test"),
            Some((Emphasis(vec![Text("w".to_string())]), 3))
        );

        assert_eq!(
            parse_emphasis("_w__ testing things test"),
            Some((Emphasis(vec![Text("w".to_string())]), 3))
        );
    }

    #[test]
    fn no_false_positives() {
        assert_eq!(
            parse_emphasis("__ testing things test"),
            None
        );
        assert_eq!(
            parse_emphasis("_ test"),
            None
        );
    }

    #[test]
    fn no_early_matching() {
        assert_eq!(
            parse_emphasis("were _testing things_ test"),
            None
        );
        assert_eq!(
            parse_emphasis("were *testing things* test"),
            None
        );
    }
}

