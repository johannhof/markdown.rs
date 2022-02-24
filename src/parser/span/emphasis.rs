use parser::span::parse_spans;
use parser::Span;
use parser::Span::Emphasis;

/// Assumes that the first character of `text` is either * or _. Also assumes that whitespace
/// escape for that first character has already been checked. Also assumes that `text` doesn't
/// start with ** or __.
pub fn parse_emphasis(text: &str) -> Option<(Span, usize)> {
    // We can assume that the first character is either * or _ if this parsing function
    // has been called.
    let mut chars = text.chars().enumerate().peekable();
    let emphasis_char = chars.next().unwrap().1;
    
    // Find the next emphasis_char, ignoring escaped characters
    let mut prev_was_whitespace = false;
    let mut end_index = None;
    while let Some((i, c)) = chars.next() {
        if c == emphasis_char {
            // Check if this is whitespace-escaped!
            let mut is_escaped = false;
            if prev_was_whitespace {
                if let Some((_, c2)) = chars.peek() {
                    is_escaped = c2.is_whitespace();
                }
            }
            if !is_escaped {
                end_index = Some(i);
                break;
            }
        } else if c == '\\' {
            chars.next();
        }
        prev_was_whitespace = c.is_whitespace();
    }
    let inner_text = &text[1..end_index?];
    if inner_text.len() != 0 {
        Some((Emphasis(parse_spans(inner_text)), inner_text.len() + 2))
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::parse_emphasis;
    use parser::Span::{Emphasis, Text, Literal};

    #[test]
    fn finds_emphasis() {
        assert_eq!(
            parse_emphasis("_testing things_ test"),
            Some((Emphasis(vec![Text("testing things".to_owned())]), 16))
        );

        assert_eq!(
            parse_emphasis("*testing things* test"),
            Some((Emphasis(vec![Text("testing things".to_owned())]), 16))
        );

        assert_eq!(
            parse_emphasis("_testing things_ things_ test"),
            Some((Emphasis(vec![Text("testing things".to_owned())]), 16))
        );

        assert_eq!(
            parse_emphasis("_w_ things_ test"),
            Some((Emphasis(vec![Text("w".to_owned())]), 3))
        );

        assert_eq!(
            parse_emphasis("*w* things* test"),
            Some((Emphasis(vec![Text("w".to_owned())]), 3))
        );

        assert_eq!(
            parse_emphasis("_w__ testing things test"),
            Some((Emphasis(vec![Text("w".to_owned())]), 3))
        );
    }

    #[test]
    fn handles_escaped_emphasis() {
        assert_eq!(
            parse_emphasis("*escape\\* test*ing"),
            Some((Emphasis(vec![
                Text("escape".to_owned()),
                Literal('*'),
                Text(" test".to_owned())]), 15))
        );

        assert_eq!(
            parse_emphasis("*fake escape\\\\* test*ing"),
            Some((Emphasis(vec![Text("fake escape".to_owned()), Literal('\\')]), 15))
        );

        assert_eq!(
            parse_emphasis("_escape\\_ test_ing"),
            Some((Emphasis(vec![
                Text("escape".to_owned()),
                Literal('_'),
                Text(" test".to_owned())]), 15))
        );

        assert_eq!(
            parse_emphasis("_fake escape\\\\_ test_ing"),
            Some((Emphasis(vec![Text("fake escape".to_owned()), Literal('\\')]), 15))
        );

        assert_eq!(
            parse_emphasis("*surrounding * whitespace* escape"),
            Some((Emphasis(vec![
                Text("surrounding ".to_owned()),
                Literal('*'),
                Text(" whitespace".to_owned())]), 26))
        );

        assert_eq!(
            parse_emphasis("_surrounding _ whitespace_ escape"),
            Some((Emphasis(vec![
                Text("surrounding ".to_owned()),
                Literal('_'),
                Text(" whitespace".to_owned())]), 26))
        );

        assert_eq!(
            parse_emphasis("*not *whitespace escaped*"),
            Some((Emphasis(vec![Text("not".to_owned())]), 6))
        )
    }

    #[test]
    fn no_false_positives() {
        assert_eq!(parse_emphasis("__ testing things test"), None);
        assert_eq!(parse_emphasis("_ test"), None);
        assert_eq!(parse_emphasis("* escape \\* test"), None);
        assert_eq!(parse_emphasis("* whitespace * escape test"), None);
        assert_eq!(parse_emphasis("*overlapping * * whitespace escapes"), None);
    }
}
