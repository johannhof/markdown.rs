use parser;
use parser::Header;
use parser::Paragraph;

pub fn to_html (tokens : Vec<parser::Token>) -> String {
    let mut ret = String::new();
    for token in tokens.iter(){
        let next = match token.el {
            Header (text, level) => format!("<h{}>{}</h{}>", level, text, level),
            Paragraph (text) => format!("<p>{}</p>", text)
        };
        ret.push_str(next.as_slice())
    }
    ret
}
