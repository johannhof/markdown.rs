use parser::Block;
use parser::Block::List;
use parser::Span::Text;
use parser::span::parse_spans;

pub fn parse_list(lines: &[&str]) -> Option<(Block, usize)>{
    // if the beginning doesn't match a list don't even bother
    if lines[0].len() < 2 || &lines[0][0 .. 2] != "* " {
        return None;
    }

    // a vec holding the contents and indentation
    // of each list item
    let mut contents = vec![];

    // the content of a single list item
    let mut content = vec![];

    // keeps track of the current indentation
    let mut indent = 0;

    // counts the number of parsed lines to return
    let mut i = 0;
 
    // captures if the previous item was a newline
    // meaning the blockquote ends next if it's not
    // explicitly continued with a >
    let mut prev_newline = false;

    for line in lines {
        // stop parsing on two newlines or if the paragraph after
        // a newline isn't started with a *
        if prev_newline && (line.is_empty() || &line.trim_left()[0 .. 1] != "*") {
            break;
        }
        if line.is_empty(){
            prev_newline = true;
        }else{
            prev_newline = false;
        }
        if line.is_empty() || &line.trim_left()[0 .. 1] == "*"{
            if !content.is_empty() {
                contents.push((content, indent));
                content = vec![];
            }
            indent = 0;
        }
        for c in line.chars(){
            if c == '*' {
                for span in parse_spans(&line[indent + 1 .. line.len()]) {
                    content.push(span);
                }
                break;
            }else if c == ' ' {
                indent += 1;
            }else{
                // if the last item of the line is a text,
                if let Some(&Text(_)) = content.last() {
                    // add a whitespace between linebreaks
                    content.push(Text(" ".to_string()));
                }
                for span in parse_spans(&line){
                    content.push(span);
                }
                break;
            }
        }
        i += 1;
    }

    if i > 0 {
        return Some((List(contents), i));
    }

    return None;
}

#[cfg(test)]
mod test {
    use super::parse_list;
    use parser::Block::List;

    #[test]
    fn finds_list() {
        match parse_list(&vec!["* A list", "* is good"]) {
            Some((List(_), 2)) => (),
            _ => panic!()
        }

        match parse_list(&vec!["* A list", "* is good", "laksjdnflakdsjnf"]) {
            Some((List(_), 3)) => (),
            _ => panic!()
        }
    }

    #[test]
    fn knows_when_to_stop() {
        match parse_list(&vec!["* A list", "* is good", "", "laksjdnflakdsjnf"]) {
            Some((List(_), 3)) => (),
            _ => panic!()
        }

        match parse_list(&vec!["* A list", "", "laksjdnflakdsjnf"]) {
            Some((List(_), 2)) => (),
            _ => panic!()
        }
    }

    #[test]
    fn no_false_positives() {
        assert_eq!(
            parse_list(&vec!["test * test"]),
            None
        );
    }

    #[test]
    fn no_early_matching() {
        assert_eq!(
            parse_list(&vec!["test", "* whot", "* a list"]),
            None
        );
    }
}

