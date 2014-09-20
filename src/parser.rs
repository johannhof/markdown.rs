use regex::Regex;

#[deriving(Show)]
pub enum Block<'s> {
    Header(Vec<Atomic<'s>>, uint),
    Break,
    Paragraph(Vec<Atomic<'s>>)
}

#[deriving(Show)]
pub enum Atomic<'s> {
    Text(&'s str)
}

static SPLIT : Regex = regex!(r"\n\n");
static ATX_HEADER : Regex = regex!(r"(?P<level>#{1,6})\s(?P<text>.*)");
static SETEXT_HEADER_1 : Regex = regex!(r"(?P<text>.+)\n===+");
static SETEXT_HEADER_2 : Regex = regex!(r"(?P<text>.+)\n---+");
static BREAK : Regex = regex!(r"  ");

pub fn parse (md : &str) -> Vec<Block> {
    let mut split = SPLIT.split(md);
    let mut tokens = vec![];
    for block in split{
        match parse_block(block) {
            Some(e) => tokens.push(e),
            None => {},
        };
    }
    tokens
}

fn parse_atomics(text : &str) -> Vec<Atomic>{
    return vec![Text(text)];
}

fn parse_block (text : &str) -> Option<Block>{
    if text.is_empty(){
        return None;
    }else if ATX_HEADER.is_match(text){
        let caps = ATX_HEADER.captures(text).unwrap();
        return Some(
            Header(
                parse_atomics(caps.name("text")),
                caps.name("level").len()
                )
            );
    }else if SETEXT_HEADER_1.is_match(text){
        let caps = SETEXT_HEADER_1.captures(text).unwrap();
        return Some(
            Header(
                parse_atomics(caps.name("text")),
                1
                )
            );
    }else if SETEXT_HEADER_2.is_match(text){
        let caps = SETEXT_HEADER_2.captures(text).unwrap();
        return Some(
            Header(
                parse_atomics(caps.name("text")),
                2
                )
            );
    }else if BREAK.is_match(text){
        return Some(Break);
    }else{
        return Some(Paragraph(parse_atomics(text)));
    }
}

