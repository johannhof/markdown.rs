use regex::Regex;
use parser::parse;
use normalizer::normalize;
use parser::span::parse_spans;
use parser::{Block, Header, Paragraph, Blockquote};
use parser::{Break, Span, Text, Emphasis, Strong, Code, Link, Image};

static ATX_HEADER      : Regex = regex!(r"^(?P<level>#{1,6})\s(?P<text>.*)");
static SETEXT_HEADER_1 : Regex = regex!(r"(?P<text>.+)\n===+");
static SETEXT_HEADER_2 : Regex = regex!(r"(?P<text>.+)\n---+");
static BLOCKQUOTE      : Regex = regex!(r"(?m)^> ?");

pub fn parse_block (text : &str) -> Option<Block>{
    if text.is_empty(){
        return None;
    }else if BLOCKQUOTE.is_match(text){
        // remove top-level >s
        let caps = BLOCKQUOTE.replace_all(text, "");
        return Some(
            Blockquote(parse(caps.as_slice()))
        );
    }else if ATX_HEADER.is_match(text){
        let caps = ATX_HEADER.captures(text).unwrap();
        return Some(
            Header(
                parse_spans(caps.name("text")),
                caps.name("level").len()
                )
            );
    }else if SETEXT_HEADER_1.is_match(text){
        let caps = SETEXT_HEADER_1.captures(text).unwrap();
        return Some(
            Header(
                parse_spans(caps.name("text")),
                1
                )
            );
    }else if SETEXT_HEADER_2.is_match(text){
        let caps = SETEXT_HEADER_2.captures(text).unwrap();
        return Some(
            Header(
                parse_spans(caps.name("text")),
                2
                )
            );
    }
    return Some(Paragraph(parse_spans(text)));
}


#[test]
fn parse_atx_header_test() {
    assert_eq!(
        parse_block("### Test").unwrap(),
        Header(vec![Text("Test")], 3)
    );

    assert_eq!(
        parse_block("# Test").unwrap(),
        Header(vec![Text("Test")], 1)
    );

    assert_eq!(
        parse_block("###### Test").unwrap(),
        Header(vec![Text("Test")], 6)
    );

    assert_eq!(
        parse_block("####### Test").unwrap(),
        Paragraph(vec![Text("####### Test")])
    );
}

