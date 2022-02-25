use parser::block::parse_blocks_from_lines;
use parser::Block;
use parser::Block::Blockquote;

/// Assumes first character of first line is '>'
pub fn parse_blockquote<'a>(lines: &[&'a str]) -> Option<(Block<'a>, usize)> {
    // the content of the blockquote
    let mut content = Vec::new();

    // counts the number of parsed lines to return
    let mut i = 0;

    // captures if the previous item was a newline
    // meaning the blockquote ends next if it's not
    // explicitly continued with a >
    let mut prev_newline = false;

    for line in lines {
        // stop parsing on two newlines or if the paragraph after
        // a newline isn't started with a >
        // we continue to parse if it's just another empty line
        if prev_newline && line.len() > 0 && !line.starts_with(">") {
            break;
        }
        if line.is_empty() {
            prev_newline = true;
        } else {
            prev_newline = false;
        }
        let mut chars = line.chars();
        let begin = match chars.next() {
            Some('>') => match chars.next() {
                Some(' ') => 2,
                _ => 1,
            },
            _ => 0,
        };
        content.push(&line[begin..line.len()]);
        i += 1;
    }

    if i > 0 {
        return Some((Blockquote(parse_blocks_from_lines(&content)), i));
    }

    None
}

#[cfg(test)]
mod test {
    use super::parse_blockquote;
    use parser::Block::Blockquote;

    #[test]
    fn finds_blockquote() {
        match parse_blockquote(&vec!["> A citation", "> is good"]) {
            Some((Blockquote(_), 2)) => (),
            _ => panic!(),
        }

        match parse_blockquote(&vec!["> A citation", "> is good,", "very good"]) {
            Some((Blockquote(_), 3)) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn knows_when_to_stop() {
        match parse_blockquote(&vec!["> A citation", "> is good", "", "whatever"]) {
            Some((Blockquote(_), 3)) => (),
            _ => panic!(),
        }
    }
}
