use regex::Regex;
use parser::parse;
use parser::span::parse_spans;
use parser::Block;
use parser::Block::{Header, Paragraph, Blockquote, Hr, CodeBlock, List};
use parser::Span::{Break, Text, Emphasis, Strong, Code, Link, Image};

pub fn parse_block (text : &str) -> Option<Block>{
    let ATX_HEADER      : Regex = Regex::new(r"^(?P<level>#{1,6})\s(?P<text>.*)").unwrap();
    let SETEXT_HEADER_1 : Regex = Regex::new(r"(?P<text>.+)\n===+").unwrap();
    let SETEXT_HEADER_2 : Regex = Regex::new(r"(?P<text>.+)\n---+").unwrap();
    let HORIZONTAL_RULE : Regex = Regex::new(r"(===+)|(---+)").unwrap();
    let BLOCKQUOTE      : Regex = Regex::new(r"(?m)^> ?").unwrap();
    let CODE_BLOCK      : Regex = Regex::new(r"(?m)^ {4}").unwrap();
    let LIST            : Regex = Regex::new(r"(?m)^\* +([^\*]*)").unwrap();

    if text.is_empty(){
        return None;
    }else if CODE_BLOCK.is_match(text){
        // remove top-level spaces
        let caps = CODE_BLOCK.replace_all(text, "");
        return Some(CodeBlock(caps));
    }else if BLOCKQUOTE.is_match(text){
        // remove top-level >s
        let caps = BLOCKQUOTE.replace_all(text, "");
        return Some(
            Blockquote(parse(&caps))
        );
    }else if ATX_HEADER.is_match(text){
        let caps = ATX_HEADER.captures(text).unwrap();
        return Some(
            Header(
                parse_spans(caps.name("text").unwrap()),
                caps.name("level").unwrap().len()
                )
            );
    }else if SETEXT_HEADER_1.is_match(text){
        let caps = SETEXT_HEADER_1.captures(text).unwrap();
        return Some(
            Header(
                parse_spans(caps.name("text").unwrap()),
                1
                )
            );
    }else if SETEXT_HEADER_2.is_match(text){
        let caps = SETEXT_HEADER_2.captures(text).unwrap();
        return Some(
            Header(
                parse_spans(caps.name("text").unwrap()),
                2
                )
            );
    }else if HORIZONTAL_RULE.is_match(text){
        return Some(Hr);
    }else if LIST.is_match(text){
        return Some(parse_list(text));
    }
    return Some(Paragraph(parse_spans(text)));
}

fn parse_list(text: &str) -> Block {
    let LIST            : Regex = Regex::new(r"(?m)^\* +([^\*]*)").unwrap();

    let mut elements = vec![];
    for cap in LIST.captures_iter(text) {
        elements.push(parse_spans(cap.at(1).unwrap_or("")));
    }
    List(elements)
}


#[test]
fn parse_atx_header_test() {
    assert_eq!(
        parse_block("### Test").unwrap(),
        Header(vec![Text("Test".to_string())], 3)
    );

    assert_eq!(
        parse_block("# Test").unwrap(),
        Header(vec![Text("Test".to_string())], 1)
    );

    assert_eq!(
        parse_block("###### Test").unwrap(),
        Header(vec![Text("Test".to_string())], 6)
    );

    assert_eq!(
        parse_block("####### Test").unwrap(),
        Paragraph(vec![Text("####### Test".to_string())])
    );
}


#[test]
fn parse_blockquote_test() {
    assert_eq!(
        parse_block("> One Paragraph\n>\n> ## H2 \n>\n").unwrap(),
        Blockquote(vec![Paragraph(vec![Text("One Paragraph".to_string())]), Header(vec![Text("H2 ".to_string())], 2)])
    );

    assert_eq!(
        parse_block("> One Paragraph\n>\n> > Another blockquote\n>\n").unwrap(),
        Blockquote(vec![Paragraph(vec![Text("One Paragraph".to_string())]),
                   Blockquote(vec![Paragraph(vec![Text("Another blockquote".to_string())])])])
    );

    assert_eq!(
        parse_block("> > One Paragraph\n> >\n> > Another blockquote\n>\n").unwrap(),
        Blockquote(vec![Blockquote(vec![Paragraph(vec![Text("One Paragraph".to_string())]),
                   Paragraph(vec![Text("Another blockquote".to_string())])])])
    );

    assert_eq!(
        parse_block("> One Paragraph, just > text \n>\n").unwrap(),
        Blockquote(vec![Paragraph(vec![Text("One Paragraph, just > text ".to_string())])])
    );

    assert_eq!(
        parse_block("> One Paragraph,\n just\n > text ").unwrap(),
        Blockquote(vec![Paragraph(vec![Text("One Paragraph,\n just\n > text ".to_string())])])
    );

    assert_eq!(
        parse_block("> One Paragraph\n>\n> just > text \n>\n").unwrap(),
        Blockquote(vec![Paragraph(vec![Text("One Paragraph".to_string())]),Paragraph(vec![Text("just > text ".to_string())])])
    );
}

#[test]
fn parse_horizontal_rule_test() {
    assert_eq!(
        parse_block("--------").unwrap(),
        Hr
    );
    assert_eq!(
        parse_block("========").unwrap(),
        Hr
    );
}

