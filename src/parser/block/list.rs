use parser::Block;
use parser::Block::List;
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
    let mut content = String::new();

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
        if prev_newline && (line.is_empty() || &line[0..1] != "*") {
            break;
        }
        if line.is_empty() {
            prev_newline = true;
            if !content.is_empty() {
                contents.push((parse_spans(&content), indent));
                content = String::new();
            }
            indent = 0;
        }else{
            prev_newline = false;
        }
        for c in line.chars(){
            if c == '*' {
                content.push_str(&line[indent + 1 .. line.len()]);
                break;
            }else if c == ' ' {
                indent += 1;
            }else{
                content.push_str(line);
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

        //match parse_blockquote(&vec!["> A citation", "> is good,", "very good"]) {
            //Some((Blockquote(_), 3)) => (),
            //_ => panic!()
        //}
    }

    //#[test]
    //fn knows_when_to_stop() {
        //match parse_blockquote(&vec!["> A citation", "> is good", "", "whatever"]) {
            //Some((Blockquote(_), 3)) => (),
            //_ => panic!()
        //}
    //}

    //#[test]
    //fn no_false_positives() {
        //assert_eq!(
            //parse_blockquote(&vec!["wat > this"]),
            //None
        //);
    //}

    //#[test]
    //fn no_early_matching() {
        //assert_eq!(
            //parse_blockquote(&vec!["Hello", "> A citation", "> is good", "", "whatever"]),
            //None
        //);
    //}
}

