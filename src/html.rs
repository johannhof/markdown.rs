use parser::Block;
use parser::Block::{Header, Paragraph, Blockquote, Hr, CodeBlock, List};
use parser::Span;
use parser::Span::{Break, Text, Emphasis, Strong, Code, Link, Image};

pub fn to_html (blocks : &Vec<Block>) -> String {
    let mut ret = String::new();
    for block in blocks.iter(){
        let next = match block {
            &Header (ref elements, level) => format_header(elements, level),
            &Paragraph (ref elements) => format_paragraph(elements),
            &Blockquote (ref elements) => format_blockquote(elements),
            &CodeBlock (ref elements) => format_codeblock(elements),
            &List (ref elements) => format_list(elements),
            &Hr => format!("<hr>")
        };
        ret.push_str(&next)
    }
    ret
}

fn format_spans(elements : &Vec<Span>) -> String {
    let mut ret = String::new();
    for element in elements.iter(){
        let next = match element  {
            &Break => format!("<br>"),
            &Text(ref text) => format!("{}", text),
            &Code(ref text) => format!("<code>{}</code>", text),
            &Link(ref text, ref url, ref title) => format!("<a href='{}' title='{}'>{}</a>", url, title, text),
            &Image(ref text, ref url, ref title) => format!("<img src='{}' title='{}' alt='{}'>", url, title, text),
            &Emphasis(ref content) => format!("<em>{}</em>", format_spans(content)),
            _ => format!("")
        };
        ret.push_str(&next)
    }
    ret
}

fn format_list(elements : &Vec<Vec<Span>>) -> String{
    let mut ret = String::new();
    for li in elements{
        ret.push_str(&format!("<li>{}</li>\n", format_spans(li)))
    }
    format!("<ul>{}</ul>\n", ret)
}

fn format_codeblock(elements : &String) -> String{
    format!("<pre><code>{}</code></pre>\n", elements)
}

fn format_blockquote(elements : &Vec<Block>) -> String{
    format!("<blockquote>{}</blockquote>\n", to_html(elements))
}

fn format_paragraph(elements : &Vec<Span>) -> String{
    format!("<p>{}</p>\n", format_spans(elements))
}

fn format_header(elements : &Vec<Span>, level : usize) -> String{
    format!("<h{}>{}</h{}>\n", level, format_spans(elements), level)
}
