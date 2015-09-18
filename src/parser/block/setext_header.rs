use regex::Regex;
use parser::Block;
use parser::Block::Header;
use parser::span::parse_spans;

pub fn parse_setext_header(lines: &[&str]) -> Option<(Block, usize)> {
    let HORIZONTAL_RULE_1 = Regex::new(r"^===+$").unwrap();
    let HORIZONTAL_RULE_2 = Regex::new(r"^---+$").unwrap();

    if lines.len() > 1 {
        if HORIZONTAL_RULE_1.is_match(lines[1]){
            return Some((Header(parse_spans(lines[0]), 1), 2));
        }else if HORIZONTAL_RULE_2.is_match(lines[1]){
            return Some((Header(parse_spans(lines[0]), 2), 2));
        }
    }
    return None;
}

#[cfg(test)]
mod test {
    use super::parse_setext_header;
    use parser::Block::Header;
    use parser::Span::Text;

    #[test]
    fn finds_atx_header() {
        assert_eq!(
            parse_setext_header(&vec!["Test", "=========="]).unwrap(),
            (Header(vec![Text("Test".to_string())], 1), 2)
        );

        assert_eq!(
            parse_setext_header(&vec!["Test", "----------"]).unwrap(),
            (Header(vec![Text("Test".to_string())], 2), 2)
        );

        assert_eq!(
            parse_setext_header(&vec!["This is a test", "==="]).unwrap(),
            (Header(vec![Text("This is a test".to_string())], 1), 2)
        );

        assert_eq!(
            parse_setext_header(&vec!["This is a test", "---"]).unwrap(),
            (Header(vec![Text("This is a test".to_string())], 2), 2)
        );
    }
}

