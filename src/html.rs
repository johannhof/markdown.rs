use parser::{Element, Header, Break, Paragraph};

pub fn to_html (tokens : Vec<Element>) -> String {
    let mut ret = String::new();
    for token in tokens.iter(){
        let next = match token {
            &Header (text, level) => format!("<h{}>{}</h{}>\n", level, text, level),
            &Paragraph (text) => format!("<p>{}</p>\n", text),
            &Break => format!("<br>\n")
        };
        ret.push_str(next.as_slice())
    }
    ret
}
