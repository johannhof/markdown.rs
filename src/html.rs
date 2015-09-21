use parser::Block;
use parser::Block::{Header, Paragraph, Blockquote, Hr, CodeBlock, List, Raw};
use parser::Span;
use parser::Span::{Break, Text, Emphasis, Strong, Code, Link, Image};

// takes a number of elements and returns their collective text as a slug
fn slugify(elements : &Vec<Span>) -> String {
    let mut ret = String::new();

    for el in elements {
        let next = match el {
            &Break => "".to_string(),
            &Text(ref text)
            | &Link(ref text, _, _)
            | &Image(ref text, _, _)
            | &Code(ref text) => text.trim().replace(" ", "_").to_lowercase().to_string(),
            &Strong(ref content) | &Emphasis(ref content) => slugify(content),
        };
        if !ret.is_empty(){
            ret.push('_');
        }
        ret.push_str(&next);
    }

    ret
}

pub fn to_html (blocks : &Vec<Block>) -> String {
    let mut ret = String::new();
    for block in blocks.iter(){
        let next = match block {
            &Header (ref elements, level) => format_header(elements, level),
            &Paragraph (ref elements) => format_paragraph(elements),
            &Blockquote (ref elements) => format_blockquote(elements),
            &CodeBlock (ref elements) => format_codeblock(elements),
            &List (ref elements) => format_list(elements),
            &Raw (ref elements) => elements.to_string(),
            &Hr => format!("<hr>")
        };
        ret.push_str(&next)
    }
    ret = ret.trim().to_string();
    ret.push('\n');
    ret
}

fn format_spans(elements : &Vec<Span>) -> String {
    let mut ret = String::new();
    for element in elements.iter(){
        let next = match element  {
            &Break => format!("<br />"),
            &Text(ref text) => format!("{}", &escape(text)),
            &Code(ref text) => format!("<code>{}</code>", &escape(text)),
            &Link(ref text, ref url, None) => format!("<a href='{}'>{}</a>", &escape(url), &escape(text)),
            &Link(ref text, ref url, Some(ref title)) => format!("<a href='{}' title='{}'>{}</a>", &escape(url), &escape(title), &escape(text)),
            &Image(ref text, ref url, None) => format!("<img src='{}' alt='{}' />", &escape(url), &escape(text)),
            &Image(ref text, ref url, Some(ref title)) => format!("<img src='{}' title='{}' alt='{}' />", &escape(url), &escape(title), &escape(text)),
            &Emphasis(ref content) => format!("<em>{}</em>", format_spans(content)),
            &Strong(ref content) => format!("<strong>{}</strong>", format_spans(content))
        };
        ret.push_str(&next)
    }
    ret
}

fn escape(text: &str) -> String{
    text.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace("\"", "&quot;")
        .replace(">", "&gt;")
}

fn format_list(elements : &Vec<(Vec<Span>, usize)>) -> String{
    let mut ret = String::new();
    for &(ref li, _indent) in elements{
        ret.push_str(&format!("<li>{}</li>", format_spans(li)))
    }
    format!("<ul>\n{}</ul>\n\n", ret)
}

fn format_codeblock(elements : &String) -> String{
    format!("<pre><code>{}</code></pre>\n\n", &escape(elements))
}

fn format_blockquote(elements : &Vec<Block>) -> String{
    format!("<blockquote>\n{}</blockquote>\n\n", to_html(elements))
}

fn format_paragraph(elements : &Vec<Span>) -> String{
    format!("<p>{}</p>\n\n", format_spans(elements))
}

fn format_header(elements : &Vec<Span>, level : usize) -> String{
    format!("<h{} id='{}'>{}</h{}>\n\n", level, slugify(elements), format_spans(elements), level)
}
