use parser::Block;
use parser::Block::CodeBlock;
use regex::Regex;

pub fn parse_code_block(lines: &[&str]) -> Option<(Block, usize)> {
    lazy_static! {
        static ref CODE_BLOCK_SPACES: Regex = Regex::new(r"^ {4}").unwrap();
        static ref CODE_BLOCK_TABS: Regex = Regex::new(r"^\t").unwrap();
        static ref CODE_BLOCK_BACKTICKS: Regex = Regex::new(r"```").unwrap();
    }

    let mut content = String::new();
    let mut lang: Option<String> = None;
    let mut line_number = 0;
    let mut backtick_opened = false;
    let mut backtick_closed = false;

    for line in lines {
        if !backtick_opened && CODE_BLOCK_SPACES.is_match(line) {
            if line_number > 0 && !content.is_empty() {
                content.push('\n');
            }
            // remove top-level spaces
            content.push_str(&line[4..line.len()]);
            line_number += 1;
        } else if !backtick_opened && CODE_BLOCK_TABS.is_match(line) {
            if line_number > 0 && !content.is_empty() {
                content.push('\n');
            }

            if !(line_number == 0 && line.trim().is_empty()) {
                // remove top-level spaces
                content.push_str(&line[1..line.len()]);
            }
            line_number += 1;
        } else if CODE_BLOCK_BACKTICKS.is_match(line) {
            line_number += 1;

            if !backtick_opened && !(line_number == 0 && line.get(3..).is_some()) {
                lang = Some(String::from(line.get(3..).unwrap()));
                backtick_opened = true;
            } else if backtick_opened {
                backtick_closed = true;
                break;
            }
        } else if backtick_opened {
            content.push_str(line);
            content.push('\n');

            line_number += 1;
        } else {
            break;
        }
    }

    if line_number > 0 && ((backtick_opened && backtick_closed) || !backtick_opened) {
        return Some((
            CodeBlock(lang, content.trim_matches('\n').to_owned()),
            line_number,
        ));
    }

    None
}

#[cfg(test)]
mod test {
    use super::parse_code_block;
    use parser::Block::CodeBlock;

    #[test]
    fn finds_code_block() {
        assert_eq!(
            parse_code_block(&vec!["    Test"]).unwrap(),
            ((CodeBlock(None, "Test".to_owned()), 1))
        );

        assert_eq!(
            parse_code_block(&vec!["    Test", "    this"]).unwrap(),
            ((CodeBlock(None, "Test\nthis".to_owned()), 2))
        );

        assert_eq!(
            parse_code_block(&vec!["```testlang", "Test", "this", "```"]).unwrap(),
            ((
                CodeBlock(Some(String::from("testlang")), "Test\nthis".to_owned()),
                4
            ))
        );
    }

    #[test]
    fn knows_when_to_stop() {
        assert_eq!(
            parse_code_block(&vec!["    Test", "    this", "stuff", "    now"]).unwrap(),
            ((CodeBlock(None, "Test\nthis".to_owned()), 2))
        );
    }

    #[test]
    fn no_false_positives() {
        assert_eq!(parse_code_block(&vec!["   Test"]), None);
    }

    #[test]
    fn no_early_matching() {
        assert_eq!(
            parse_code_block(&vec!["Test", "    this", "stuff", "    now"]),
            None
        );
    }
}
