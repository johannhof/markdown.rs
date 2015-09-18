use regex::Regex;
use parser::Span;
use parser::Span::Break;

pub fn parse_break(text: &str) -> Option<(Span, usize)>{
    let BREAK = Regex::new(r"^ {2}$").unwrap();

    if BREAK.is_match(text){
        return Some((Break, 2));
    }
    return None;
}

#[cfg(test)]
mod test {
    use parser::Span::Break;
    use super::parse_break;

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

