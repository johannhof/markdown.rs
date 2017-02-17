use parser::Block;
use parser::block::parse_blocks;
use parser::Block::{UnorderedList, Paragraph};
use parser::ListItem;
use regex::Regex;

pub fn parse_unordered_list(lines: &[&str]) -> Option<(Block, usize)> {
    lazy_static! {
        static ref LIST_BEGIN :Regex = Regex::new(r"^(?P<indent> *)(-|\+|\*) (?P<content>.*)").unwrap();
        static ref NEW_PARAGRAPH :Regex = Regex::new(r"^ +").unwrap();
        static ref INDENTED :Regex = Regex::new(r"^ {0,4}(?P<content>.*)").unwrap();
    }

    // if the beginning doesn't match a list don't even bother
    if !LIST_BEGIN.is_match(lines[0]) {
        return None;
    }

    // a vec holding the contents and indentation
    // of each list item
    let mut contents = vec![];
    let mut prev_newline = false;
    let mut is_paragraph = false;

    // counts the number of parsed lines to return
    let mut i = 0;

    let mut line_iter = lines.iter();
    let mut line = line_iter.next();

    // loop for list items
    loop {
        if line.is_none() || !LIST_BEGIN.is_match(line.unwrap()) {
            break;
        }
        if prev_newline {
            is_paragraph = true;
            prev_newline = false;
        }

        let caps = LIST_BEGIN.captures(line.unwrap()).unwrap();

        let mut content = caps.name("content").unwrap().as_str().to_owned();
        let last_indent = caps.name("indent").unwrap().as_str().len();
        i += 1;

        // parse additional lines of the listitem
        loop {
            line = line_iter.next();

            if line.is_none() || (prev_newline && !NEW_PARAGRAPH.is_match(line.unwrap())) {
                break;
            }

            if LIST_BEGIN.is_match(line.unwrap()) {
                let caps = LIST_BEGIN.captures(line.unwrap()).unwrap();
                let indent = caps.name("indent").unwrap().as_str().len();
                if indent < 2 || indent <= last_indent {
                    break;
                }
            }

            // newline means we start a new paragraph
            if line.unwrap().is_empty() {
                prev_newline = true;
            } else {
                prev_newline = false;
            }

            content.push('\n');
            let caps = INDENTED.captures(line.unwrap()).unwrap();
            content.push_str(&caps.name("content").unwrap().as_str());

            i += 1;
        }
        contents.push(parse_blocks(&content));
    }

    let mut list_contents = vec![];

    for c in contents {
        if is_paragraph || c.len() > 1 {
            list_contents.push(ListItem::Paragraph(c));
        } else if let Paragraph(content) = c[0].clone() {
            list_contents.push(ListItem::Simple(content));
        }
    }

    if i > 0 {
        return Some((UnorderedList(list_contents), i));
    }

    None
}

#[cfg(test)]
mod test {
    use super::parse_unordered_list;
    use parser::Block::UnorderedList;

    #[test]
    fn finds_list() {
        match parse_unordered_list(&vec!["* A list", "* is good"]) {
            Some((UnorderedList(_), 2)) => (),
            x => panic!("Found {:?}", x),
        }

        match parse_unordered_list(&vec!["* A list", "* is good", "laksjdnflakdsjnf"]) {
            Some((UnorderedList(_), 3)) => (),
            x => panic!("Found {:?}", x),
        }
    }

    #[test]
    fn knows_when_to_stop() {
        match parse_unordered_list(&vec!["* A list", "* is good", "", "laksjdnflakdsjnf"]) {
            Some((UnorderedList(_), 3)) => (),
            x => panic!("Found {:?}", x),
        }

        match parse_unordered_list(&vec!["* A list", "", "laksjdnflakdsjnf"]) {
            Some((UnorderedList(_), 2)) => (),
            x => panic!("Found {:?}", x),
        }
    }

    #[test]
    fn no_false_positives() {
        assert_eq!(parse_unordered_list(&vec!["test * test"]), None);
    }

    #[test]
    fn no_early_matching() {
        assert_eq!(parse_unordered_list(&vec!["test", "* whot", "* a list"]),
                   None);
    }
}
