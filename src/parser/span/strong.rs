use parser::span::parse_spans;
use parser::Span;
use parser::Span::Strong;

/// Assumes that the first two characters of `text` are either ** or __.
pub fn parse_strong(text: &str) -> Option<(Span, usize)> {
    // We can assume that the first character is either * or _ if this parsing function
    // has been called.
    let mut chars = text.chars().enumerate().peekable();
    let emphasis_char = chars.next().unwrap().1;
    chars.next();
    
    // Find the next emphasis_char, ignoring escaped characters
    let mut end_index = None;
    let mut first_in_pair_found = false;
    while let Some((i, c)) = chars.next() {
        if c == emphasis_char {
            if first_in_pair_found {
                end_index = Some(i - 1);
                break;
            } else {
                first_in_pair_found = true;
            }
        } else {
            first_in_pair_found = false;
            if c == '\\' {
                chars.next();
            }
        }
    }
    let inner_text = &text[2..end_index?];
    if inner_text.len() != 0 {
        Some((Strong(parse_spans(inner_text)), inner_text.len() + 4))
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::parse_strong;
    use parser::Span::{Strong, Text, Literal};

    #[test]
    fn finds_strong() {
        assert_eq!(
            parse_strong("__testing things__ test"),
            Some((Strong(vec![Text("testing things".into())]), 18))
        );

        assert_eq!(
            parse_strong("**testing things** test"),
            Some((Strong(vec![Text("testing things".into())]), 18))
        );

        assert_eq!(
            parse_strong("__testing things__ things__ test"),
            Some((Strong(vec![Text("testing things".into())]), 18))
        );

        assert_eq!(
            parse_strong("__w__ things_ test"),
            Some((Strong(vec![Text("w".into())]), 5))
        );

        assert_eq!(
            parse_strong("**w** things** test"),
            Some((Strong(vec![Text("w".into())]), 5))
        );

        assert_eq!(
            parse_strong("__w___ testing things test"),
            Some((Strong(vec![Text("w".into())]), 5))
        );
    }

    #[test]
    fn handles_escaped_strong() {
        assert_eq!(
            parse_strong("**escape\\** test**ing"),
            Some((Strong(vec![
                Text("escape".into()),
                Literal('*'),
                Text("* test".into())]), 18))
        );

        assert_eq!(
            parse_strong("**fake escape\\\\** test**ing"),
            Some((Strong(vec![Text("fake escape".into()), Literal('\\')]), 17))
        );

        assert_eq!(
            parse_strong("__escape\\__ test__ing"),
            Some((Strong(vec![
                Text("escape".into()),
                Literal('_'),
                Text("_ test".into())]), 18))
        );

        assert_eq!(
            parse_strong("__fake escape\\\\__ test__ing"),
            Some((Strong(vec![Text("fake escape".into()), Literal('\\')]), 17))
        );
    }

    #[test]
    fn no_false_positives() {
        assert_eq!(parse_strong("__ testing things test"), None);
        assert_eq!(parse_strong("__testing things** test"), None);
        assert_eq!(parse_strong("____ testing things test"), None);
        assert_eq!(parse_strong("** test"), None);
        assert_eq!(parse_strong("**** test"), None);
    }

    #[test]
    fn no_early_matching() {
        assert_eq!(parse_strong("were __testing things__ test"), None);
        assert_eq!(parse_strong("were **testing things** test"), None);
    }
}
