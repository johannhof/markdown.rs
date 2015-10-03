use parser::Block;
use parser::Block::{UnorderedList, Paragraph};
use parser::ListItem;
use parser::Span::{Text, Break};
use parser::span::parse_spans;
use regex::Regex;

pub fn parse_unordered_list(lines: &[&str]) -> Option<(Block, usize)>{
    let list_begin = Regex::new(r"^(?P<indent> *)(-|\+|\*) (?P<content>.*)").unwrap();
    let new_paragraph = Regex::new(r"^ {4}(?P<content>.*)").unwrap();

    // if the beginning doesn't match a list don't even bother
    if !list_begin.is_match(lines[0]) {
        return None;
    }

    // a vec holding the contents and indentation
    // of each list item
    let mut contents = vec![];
    let mut is_paragraph = false;

    // counts the number of parsed lines to return
    let mut i = 0;

    let mut line_iter = lines.iter();
    let mut line = line_iter.next();

    // loop for list items
    loop {
        let mut content = vec![];
        let mut paragraph = vec![];
        let mut prev_newline = false;

        if line.is_none() || !list_begin.is_match(line.unwrap()) {
            break;
        }

        let caps = list_begin.captures(line.unwrap()).unwrap();

        let indent = caps.name("indent").unwrap().len();
        for span in parse_spans(&caps.name("content").unwrap()){
            paragraph.push(span);
        }
        i += 1;

        // parse additional lines of the listitem
        loop {
            line = line_iter.next();

            // break if:
            // the line is a list item

            if line.is_none() {
                break;
            }

            if list_begin.is_match(line.unwrap()) {
                break;
            }

            // there is no indent hinting at another paragraph
            if prev_newline && !new_paragraph.is_match(line.unwrap()) {
                break;
            } else if prev_newline{
                is_paragraph = true;
                content.push(paragraph);
                paragraph = vec![];
            }

            // newline means we start a new paragraph
            if(line.unwrap().is_empty()){
                prev_newline = true;
            }else{
                prev_newline = false;
            }

            let spans = parse_spans(&line.unwrap());

            // add a whitespace between linebreaks
            // except when we have a break element or nothing
            match (paragraph.last(), spans.first()) {
                (Some(&Break), _) => {},
                (_, None) => {},
                (None, _) => {},
                _ => paragraph.push(Text(" ".to_string()))
            }

            for span in spans{
                paragraph.push(span);
            }

            i += 1;
        }
        content.push(paragraph);
        contents.push((content, indent));
    }

    let mut list_contents = vec![];

    for (c, indent) in contents {
        if is_paragraph {
            let content = c.into_iter().map(|p| Paragraph(p)).collect();
            list_contents.push(ListItem::Paragraph(content, indent));
        }else{
            list_contents.push(ListItem::Simple(c[0].clone(), indent));
        }
    }

    if i > 0 {
        return Some((UnorderedList(list_contents), i));
    }

    return None;
}

#[cfg(test)]
mod test {
    use super::parse_unordered_list;
    use parser::Block::UnorderedList;

    #[test]
    fn finds_list() {
        match parse_unordered_list(&vec!["* A list", "* is good"]) {
            Some((UnorderedList(_), 2)) => (),
            x => panic!("Found {:?}", x)
        }

        match parse_unordered_list(&vec!["* A list", "* is good", "laksjdnflakdsjnf"]) {
            Some((UnorderedList(_), 3)) => (),
            x => panic!("Found {:?}", x)
        }
    }

    #[test]
    fn knows_when_to_stop() {
        match parse_unordered_list(&vec!["* A list", "* is good", "", "laksjdnflakdsjnf"]) {
            Some((UnorderedList(_), 3)) => (),
            x => panic!("Found {:?}", x)
        }

        match parse_unordered_list(&vec!["* A list", "", "laksjdnflakdsjnf"]) {
            Some((UnorderedList(_), 2)) => (),
            x => panic!("Found {:?}", x)
        }
    }

    #[test]
    fn no_false_positives() {
        assert_eq!(
            parse_unordered_list(&vec!["test * test"]),
            None
        );
    }

    #[test]
    fn no_early_matching() {
        assert_eq!(
            parse_unordered_list(&vec!["test", "* whot", "* a list"]),
            None
        );
    }
}

