use parser::{Block, Atomic, Header, Text, Break, Paragraph};

pub fn to_html (blocks : Vec<Block>) -> String {
    let mut ret = String::new();
    for block in blocks.iter(){
        let next = match block {
            &Header (ref elements, level) => format_header(elements, level),
            &Paragraph (ref elements) => format_paragraph(elements),
            &Break => format!("<br>\n")
        };
        ret.push_str(next.as_slice())
    }
    ret
}

fn format_atomics(elements : &Vec<Atomic>) -> String {
    let mut ret = String::new();
    for element in elements.iter(){
        let next = match element  {
            &Text(text) => text,
        };
        ret.push_str(next.as_slice())
    }
    ret
}

fn format_paragraph(elements : &Vec<Atomic>) -> String{
    format!("<p>{}</p>\n", format_atomics(elements))
}

fn format_header(elements : &Vec<Atomic>, level : uint) -> String{
    format!("<h{}>{}</h{}>\n", level, format_atomics(elements), level)
}
