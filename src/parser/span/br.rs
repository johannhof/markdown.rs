use parser::Span;
use parser::Span::Break;
use regex::Regex;

pub fn parse_break(text: &str) -> Option<(Span, usize)> {
    lazy_static! {
        static ref BR: Regex = Regex::new(r"^ {2}$").unwrap();
    }

    if BR.is_match(text) {
        return Some((Break, 2));
    }
    None
}

#[cfg(test)]
mod test {
    use super::parse_break;
    use parser::Span::Break;

    #[test]
    fn finds_breaks() {
        assert_eq!(parse_break("  "), Some((Break, 2)));
    }

    #[test]
    fn no_false_positives() {
        assert_eq!(parse_break("this is a test  "), None);
        assert_eq!(parse_break(" "), None);
        assert_eq!(parse_break("  a"), None);
    }
}
