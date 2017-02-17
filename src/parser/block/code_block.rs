use regex::Regex;
use parser::Block;
use parser::Block::CodeBlock;

pub fn parse_code_block(lines: &[&str]) -> Option<(Block, usize)> {
    lazy_static! {
        static ref CODE_BLOCK_SPACES :Regex = Regex::new(r"^ {4}").unwrap();
        static ref CODE_BLOCK_TABS :Regex = Regex::new(r"^\t").unwrap();
    }

    let mut content = String::new();
    let mut i = 0;
    for line in lines {
        if CODE_BLOCK_SPACES.is_match(line) {
            if i > 0 && !content.is_empty() {
                content.push('\n');
            }
            // remove top-level spaces
            content.push_str(&line[4..line.len()]);
            i += 1;
        } else if CODE_BLOCK_TABS.is_match(line) {
            if i > 0 && !content.is_empty() {
                content.push('\n');
            }

            if !(i == 0 && line.trim().is_empty()) {
                // remove top-level spaces
                content.push_str(&line[1..line.len()]);
            }
            i += 1;
        } else {
            break;
        }
    }
    if i > 0 {
        return Some((CodeBlock(content.trim_matches('\n').to_owned()), i));
    }
    None
}

#[cfg(test)]
mod test {
    use super::parse_code_block;
    use parser::Block::CodeBlock;

    #[test]
    fn finds_code_block() {
        assert_eq!(parse_code_block(&vec!["    Test"]).unwrap(),
                   ((CodeBlock("Test".to_owned()), 1)));

        assert_eq!(parse_code_block(&vec!["    Test", "    this"]).unwrap(),
                   ((CodeBlock("Test\nthis".to_owned()), 2)));
    }

    #[test]
    fn knows_when_to_stop() {
        assert_eq!(parse_code_block(&vec!["    Test", "    this", "stuff", "    now"]).unwrap(),
                   ((CodeBlock("Test\nthis".to_owned()), 2)));
    }

    #[test]
    fn no_false_positives() {
        assert_eq!(parse_code_block(&vec!["   Test"]), None);
    }

    #[test]
    fn no_early_matching() {
        assert_eq!(parse_code_block(&vec!["Test", "    this", "stuff", "    now"]),
                   None);
    }
}
