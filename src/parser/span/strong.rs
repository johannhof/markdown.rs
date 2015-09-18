use regex::Regex;
use parser::span::parse_spans;
use parser::Span;
use parser::Span::Strong;

pub fn parse_strong(text: &str) -> Option<(Span, usize)>{
    let STRONG_UNDERSCORE = Regex::new(r"^__(?P<text>.+?)__").unwrap();
    let STRONG_STAR = Regex::new(r"^\*\*(?P<text>.+?)\*\*").unwrap();

    if STRONG_UNDERSCORE.is_match(text){
        let caps = STRONG_UNDERSCORE.captures(text).unwrap();
        let t = caps.name("text").unwrap();
        return Some((Strong(parse_spans(t)), t.len() + 4));
    }else if STRONG_STAR.is_match(text){
        let caps = STRONG_STAR.captures(text).unwrap();
        let t = caps.name("text").unwrap();
        return Some((Strong(parse_spans(t)), t.len() + 4));
    }
    return None;
}

#[cfg(test)]
mod test {
    use parser::Span::{Text, Strong};
    use super::parse_strong;

    #[test]
    fn finds_strong() {
        assert_eq!(
            parse_strong("__testing things__ test"),
            Some((Strong(vec![Text("testing things".to_string())]), 18))
        );

        assert_eq!(
            parse_strong("**testing things** test"),
            Some((Strong(vec![Text("testing things".to_string())]), 18))
        );

        assert_eq!(
            parse_strong("__testing things__ things__ test"),
            Some((Strong(vec![Text("testing things".to_string())]), 18))
        );

        assert_eq!(
            parse_strong("__w__ things_ test"),
            Some((Strong(vec![Text("w".to_string())]), 5))
        );

        assert_eq!(
            parse_strong("**w** things** test"),
            Some((Strong(vec![Text("w".to_string())]), 5))
        );

        assert_eq!(
            parse_strong("__w___ testing things test"),
            Some((Strong(vec![Text("w".to_string())]), 5))
        );
    }

    #[test]
    fn no_false_positives() {
        assert_eq!(
            parse_strong("__ testing things test"),
            None
        );
        assert_eq!(
            parse_strong("__testing things** test"),
            None
        );
        assert_eq!(
            parse_strong("____ testing things test"),
            None
        );
        assert_eq!(
            parse_strong("** test"),
            None
        );
        assert_eq!(
            parse_strong("**** test"),
            None
        );
    }

    #[test]
    fn no_early_matching() {
        assert_eq!(
            parse_strong("were __testing things__ test"),
            None
        );
        assert_eq!(
            parse_strong("were **testing things** test"),
            None
        );
    }
}
