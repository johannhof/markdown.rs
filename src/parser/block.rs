use regex::Regex;
use parser::span::parse_spans;
use parser::{Block, Header, Break, Paragraph};

// Block Patters
static ATX_HEADER      : Regex = regex!(r"(?P<level>#{1,6})\s(?P<text>.*)");
static SETEXT_HEADER_1 : Regex = regex!(r"(?P<text>.+)\n===+");
static SETEXT_HEADER_2 : Regex = regex!(r"(?P<text>.+)\n---+");
static BREAK           : Regex = regex!(r"  ");

pub fn parse_block (text : &str) -> Option<Block>{
    if text.is_empty(){
        return None;
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
    }else if BREAK.is_match(text){
        return Some(Break);
    }
    return Some(Paragraph(parse_spans(text)));
}

