use parser::Block;
use parser::Block::CodeBlock;

pub fn parse_code_block<'a>(lines: &[&'a str]) -> Option<(Block<'a>, usize)> {

    let mut content = Vec::new();
    let mut lang: Option<&str> = None;
    let mut line_number = 0;
    let mut backtick_opened = false;
    let mut backtick_closed = false;
    for line in lines {
        if !backtick_opened && line.starts_with("    ") {
            // remove top-level spaces
            content.push(&line[4..]);
            line_number += 1;
        } else if !backtick_opened && line.starts_with('\t') {
            if !(line_number == 0 && line.trim().is_empty()) {
                // remove top-level spaces
                content.push(&line[1..]);
            }
            line_number += 1;
        } else if line.starts_with("```") {
            line_number += 1;

            if !backtick_opened && !(line_number == 0 && line.len() > 3) {
                lang = Some(&line[3..]);
                backtick_opened = true;
            } else if backtick_opened {
                backtick_closed = true;
                break;
            }
        } else if backtick_opened {
            content.push(line);

            line_number += 1;
        } else {
            break;
        }
    }
    
    while let Some(s) = content.first() {
        if s.trim().is_empty() {
            content.remove(0);
        } else {
            break;
        }
    }

    while let Some(s) = content.last() {
        if s.trim().is_empty() {
            content.pop();
        } else {
            break;
        }
    }

    if line_number > 0 && ((backtick_opened && backtick_closed) || !backtick_opened) {
        return Some((
            CodeBlock(lang, content),
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
            ((CodeBlock(None, vec!["Test"]), 1))
        );

        assert_eq!(
            parse_code_block(&vec!["    Test", "    this"]).unwrap(),
            ((CodeBlock(None, vec!["Test", "this"]), 2))
        );

        assert_eq!(
            parse_code_block(&vec!["```testlang", "Test", "this", "```"]).unwrap(),
            ((CodeBlock(Some("testlang"), vec!["Test", "this"]), 4))
        );
    }

    #[test]
    fn knows_when_to_stop() {
        assert_eq!(
            parse_code_block(&vec!["    Test", "    this", "stuff", "    now"]).unwrap(),
            ((CodeBlock(None, vec!["Test", "this"]), 2))
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
