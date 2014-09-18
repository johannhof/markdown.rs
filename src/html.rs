use parser;
use parser::Element;
use parser::Header;
use parser::Paragraph;

pub fn to_html (tokens : Vec<Element>) -> String {
    let mut ret = String::new();
    for token in tokens.iter(){
        let next = match token {
            &Header (text, level) => format!("<h{}>{}</h{}>\n", level, text, level),
            &Paragraph (text) => format!("<p>{}</p>\n", text)
        };
        ret.push_str(next.as_slice())
    }
    ret
}
