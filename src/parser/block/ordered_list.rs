use parser::block::parse_blocks;
use parser::Block;
use parser::Block::{OrderedList, Paragraph};
use parser::{ListItem, OrderedListType};
use regex::Regex;

pub fn parse_ordered_list(lines: &[&str]) -> Option<(Block, usize)> {
    lazy_static! {
        static ref LIST_BEGIN: Regex =
            Regex::new(r"^(?P<indent> *)(?P<numbering>[0-9.]+|[aAiI]+\.) (?P<content>.*)").unwrap();
        static ref NEW_PARAGRAPH: Regex = Regex::new(r"^ +").unwrap();
        static ref INDENTED: Regex = Regex::new(r"^ {0,4}(?P<content>.*)").unwrap();
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
    let mut list_num_opt = None;

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
        //We use the first list type found
        // TODO: utf-8 safe?
        list_num_opt = list_num_opt
            .or_else(|| Some(caps.name("numbering").unwrap().as_str()[0..1].to_owned()));
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

    for c in contents.into_iter().filter(|x| !x.is_empty()) {
        if is_paragraph || c.len() > 1 {
            list_contents.push(ListItem::Paragraph(c));
        } else if let Paragraph(content) = c[0].clone() {
            list_contents.push(ListItem::Simple(content));
        }
    }

    if i > 0 {
        let list_num = list_num_opt.unwrap_or("1".to_string());
        return Some((
            OrderedList(list_contents, OrderedListType::from_str(&list_num)),
            i,
        ));
    }

    None
}

#[cfg(test)]
mod test {
    use super::parse_ordered_list;
    use parser::Block::OrderedList;
    use parser::ListItem::Paragraph;
    use parser::OrderedListType;

    #[test]
    fn finds_list() {
        match parse_ordered_list(&vec!["1. A list", "2. is good"]) {
            Some((OrderedList(_, lt), 2)) if lt == OrderedListType::Numeric => (),
            x => panic!("Found {:?}", x),
        }

        match parse_ordered_list(&vec!["a. A list", "b. is good", "laksjdnflakdsjnf"]) {
            Some((OrderedList(_, lt), 3)) if lt == OrderedListType::Lowercase => (),
            x => panic!("Found {:?}", x),
        }

        match parse_ordered_list(&vec!["A. A list", "B. is good", "laksjdnflakdsjnf"]) {
            Some((OrderedList(_, lt), 3)) if lt == OrderedListType::Uppercase => (),
            x => panic!("Found {:?}", x),
        }
    }

    #[test]
    fn knows_when_to_stop() {
        match parse_ordered_list(&vec!["i. A list", "ii. is good", "", "laksjdnflakdsjnf"]) {
            Some((OrderedList(_, lt), 3)) if lt == OrderedListType::LowercaseRoman => (),
            x => panic!("Found {:?}", x),
        }

        match parse_ordered_list(&vec!["I. A list", "", "laksjdnflakdsjnf"]) {
            Some((OrderedList(_, lt), 2)) if lt == OrderedListType::UppercaseRoman => (),
            x => panic!("Found {:?}", x),
        }
    }

    #[test]
    fn multi_level_list() {
        match parse_ordered_list(&vec![
            "1. A list",
            "     1.1. One point one",
            "     1.2. One point two",
        ]) {
            Some((OrderedList(ref items, lt), 3)) if lt == OrderedListType::Numeric => {
                match &items[0] {
                    &Paragraph(ref items) => match &items[1] {
                        &OrderedList(_, ref lt1) if lt1 == &OrderedListType::Numeric => (),
                        x => panic!("Found {:?}", x),
                    },
                    x => panic!("Found {:?}", x),
                }
            }
            x => panic!("Found {:?}", x),
        }
    }

    #[test]
    fn no_false_positives() {
        assert_eq!(parse_ordered_list(&vec!["test 1. test"]), None);
    }

    #[test]
    fn no_early_matching() {
        assert_eq!(
            parse_ordered_list(&vec!["test", "1. not", "2. a list"]),
            None
        );
    }
}
