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

pub fn parse (md : &'static str) -> Vec<Token> {
    let mut v = md.split('\n');
    let mut tokens = vec![];
    for x in v{
        let re = regex!(r"\s*(?P<level>#{1,6})(?P<text>.*)");
        if re.is_match(x){
            let caps = re.captures(x).unwrap();
            tokens.push(Token{
                el : Header
                (
                    caps.name("text"),
                    caps.name("level").len()
                ),
                md: x
            });
        }else{
            tokens.push(Token{
                el : Paragraph ( x),
                md: x
            });
        }
    }
    tokens
}

