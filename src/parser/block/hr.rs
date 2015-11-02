use regex::Regex;
use parser::Block;
use parser::Block::Hr;

pub fn parse_hr(lines: &[&str]) -> Option<(Block, usize)> {
    let horizontal_rule = Regex::new(r"^(===+)$|^(---+)$").unwrap();

    if horizontal_rule.is_match(lines[0]) {
        return Some((Hr, 1));
    }
    None
}

#[cfg(test)]
mod test {
    use super::parse_hr;
    use parser::Block::Hr;

    #[test]
    fn finds_hr() {
        assert_eq!(parse_hr(&vec!["-------"]).unwrap(), (Hr, 1));
        assert_eq!(parse_hr(&vec!["---"]).unwrap(), (Hr, 1));
        assert_eq!(parse_hr(&vec!["----------------------------"]).unwrap(),
                   (Hr, 1));
        assert_eq!(parse_hr(&vec!["-------", "abc"]).unwrap(), (Hr, 1));

        assert_eq!(parse_hr(&vec!["======="]).unwrap(), (Hr, 1));
        assert_eq!(parse_hr(&vec!["==="]).unwrap(), (Hr, 1));
        assert_eq!(parse_hr(&vec!["============================"]).unwrap(),
                   (Hr, 1));
        assert_eq!(parse_hr(&vec!["=======", "abc"]).unwrap(), (Hr, 1));
    }

    #[test]
    fn no_false_positives() {
        assert_eq!(parse_hr(&vec!["a-------"]), None);
        assert_eq!(parse_hr(&vec!["--- a"]), None);
        assert_eq!(parse_hr(&vec!["--a-"]), None);
        assert_eq!(parse_hr(&vec!["-------====--------------"]), None);

        assert_eq!(parse_hr(&vec!["a======"]), None);
        assert_eq!(parse_hr(&vec!["=== a"]), None);
        assert_eq!(parse_hr(&vec!["==a="]), None);
        assert_eq!(parse_hr(&vec!["=======---================="]), None);
    }
}
