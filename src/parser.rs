use regex::Regex;

#[deriving(Show)]
pub struct Token {
    pub el : Element,
    pub md : &'static str,
}

#[deriving(Show)]
pub enum Element {
    Header(&'static str, uint),
    Paragraph(&'static str)
}

static SPLIT : Regex = regex!(r"\n");
static HEADER : Regex = regex!(r"\s*(?P<level>#{1,6})(?P<text>.*)");

//fn is_header(&iter : Iterator<String>) -> bool{

//}

pub fn parse (md : &'static str) -> Vec<Token> {
    let mut split = SPLIT.split(md).peekable();
    let mut tokens = vec![];
    for text in split{
      tokens.push(parse_segment(text));
    }
    tokens
}

fn parse_segment(text : &'static str) -> Token{
    if HEADER.is_match(text){
        let caps = HEADER.captures(text).unwrap();
        return Token{
            el : Header
            (
                caps.name("text"),
                caps.name("level").len()
            ),
            md: text
        };
    }else{
      return Token{
          el : Paragraph(text),
          md: text
      };
    }
}
