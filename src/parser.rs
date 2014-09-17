#[deriving(Show)]
pub struct Token {
    kind : &'static str,
    md : &'static str
}

pub fn parse (md : &'static str) -> Vec<Token> {
    let mut v = md.split('\n');
    let mut tokens = vec![];
    for x in v{
      let re = regex!(r"\s*#.*");
      if re.is_match(x){
        tokens.push(Token{
          kind: "Header",
          md: x
        });
      }else{
        tokens.push(Token{
          kind: "Unknown",
          md: x
        });
      }
    }
    tokens
}

