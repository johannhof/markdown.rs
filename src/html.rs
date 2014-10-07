use parser::{Block, Header, Paragraph};
use parser::{Break, Span, Text, Code, Link, Image, Emphasis};

pub fn to_html (blocks : Vec<Block>) -> String {
    let mut ret = String::new();
    for block in blocks.iter(){
        let next = match block {
            &Header (ref elements, level) => format_header(elements, level),
            &Paragraph (ref elements) => format_paragraph(elements)
        };
        ret.push_str(next.as_slice())
    }
    ret
}

fn format_spans(elements : &Vec<Span>) -> String {
    let mut ret = String::new();
    for element in elements.iter(){
        let next = match element  {
            &Break => format!("<br>"),
            &Text(text) => format!("{}", text),
            &Code(text) => format!("<code>{}</code>", text),
            &Link(text, url, title) => format!("<a href='{}' title='{}'>{}</a>", url, title, text),
            &Image(text, url, title) => format!("<img src='{}' title='{}' alt='{}'>", url, title, text),
            &Emphasis(ref content) => format!("<em>{}</em>", format_spans(content)),
            _ => format!("")
        };
        ret.push_str(next.as_slice())
    }
    ret
}

fn format_paragraph(elements : &Vec<Span>) -> String{
    format!("<p>{}</p>\n", format_spans(elements))
}

fn format_header(elements : &Vec<Span>, level : uint) -> String{
    format!("<h{}>{}</h{}>\n", level, format_spans(elements), level)
}
