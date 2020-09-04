use parser::Block;
use parser::Block::{
    Blockquote, CodeBlock, Header, Hr, OrderedList, Paragraph, Raw, UnorderedList,
};
use parser::Span::{Break, Code, Emphasis, Image, Link, Literal, Strong, Text};
use parser::{ListItem, OrderedListType, Span};
use regex::Regex;

// takes a number of elements and returns their collective text as a slug
fn slugify(elements: &[Span]) -> String {
    let mut ret = String::new();

    for el in elements {
        let next = match *el {
            Break => "".to_owned(),
            Literal(character) => character.to_string(),
            Text(ref text) | Image(ref text, _, _) | Code(ref text) => {
                text.trim().replace(" ", "_").to_lowercase().to_owned()
            }
            Link(ref content, _, _) | Strong(ref content) | Emphasis(ref content) => {
                slugify(content)
            }
        };
        if !ret.is_empty() {
            ret.push('_');
        }
        ret.push_str(&next);
    }

    ret
}

pub fn to_html(blocks: &[Block]) -> String {
    let mut ret = String::new();
    for block in blocks.iter() {
        let next = match *block {
            Header(ref elements, level) => format_header(elements, level),
            Paragraph(ref elements) => format_paragraph(elements),
            Blockquote(ref elements) => format_blockquote(elements),
            CodeBlock(ref lang, ref elements) => format_codeblock(lang, elements),
            UnorderedList(ref elements) => format_unordered_list(elements),
            OrderedList(ref elements, ref num_type) => format_ordered_list(elements, num_type),
            Raw(ref elements) => elements.to_owned(),
            Hr => format!("<hr>"),
        };
        ret.push_str(&next)
    }
    ret = ret.trim().to_owned();
    ret.push('\n');
    ret
}

fn format_spans(elements: &[Span]) -> String {
    let mut ret = String::new();
    for element in elements.iter() {
        let next = match *element {
            Break => format!("<br />"),
            Literal(character) => character.to_string(),
            Text(ref text) => format!("{}", &escape(text, true)),
            Code(ref text) => format!("<code>{}</code>", &escape(text, false)),
            Link(ref content, ref url, None) => format!(
                "<a href=\"{}\">{}</a>",
                &escape(url, false),
                format_spans(content)
            ),
            Link(ref content, ref url, Some(ref title)) => format!(
                "<a href=\"{}\" title=\"{}\">{}</a>",
                &escape(url, false),
                &escape(title, true),
                format_spans(content)
            ),
            Image(ref text, ref url, None) => format!(
                "<img src=\"{}\" alt=\"{}\" />",
                &escape(url, false),
                &escape(text, true)
            ),
            Image(ref text, ref url, Some(ref title)) => format!(
                "<img src=\"{}\" title=\"{}\" alt=\"{}\" />",
                &escape(url, false),
                &escape(title, true),
                &escape(text, true)
            ),
            Emphasis(ref content) => format!("<em>{}</em>", format_spans(content)),
            Strong(ref content) => format!("<strong>{}</strong>", format_spans(content)),
        };
        ret.push_str(&next)
    }
    ret
}

fn escape(text: &str, replace_entities: bool) -> String {
    lazy_static! {
        static ref AMPERSAND: Regex = Regex::new(r"&amp;(?P<x>\w+;)").unwrap();
    }

    let replaced = text
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace("\"", "&quot;")
        .replace("'", "&#8217;")
        .replace(">", "&gt;");

    // We can't do lookarounds in the regex crate to match only ampersands with
    // no entity; afterwards, so we do this ugly hack where we revert the replacement
    // everywhere it wasn't desired.
    if replace_entities {
        return AMPERSAND.replace_all(&replaced, "&$x").into_owned();
    }

    return replaced;
}

fn format_list(elements: &[ListItem], start_tag: &str, end_tag: &str) -> String {
    let mut ret = String::new();
    for list_item in elements {
        let mut content = String::new();
        match *list_item {
            ListItem::Simple(ref els) => content.push_str(&format_spans(els)),
            ListItem::Paragraph(ref paragraphs) => {
                content.push_str(&format!("\n{}", to_html(paragraphs)))
            }
        }

        ret.push_str(&format!("\n<li>{}</li>\n", content))
    }
    format!("<{}>{}</{}>\n\n", start_tag, ret, end_tag)
}

fn format_unordered_list(elements: &[ListItem]) -> String {
    format_list(elements, "ul", "ul")
}

fn format_ordered_list(elements: &[ListItem], num_type: &OrderedListType) -> String {
    if num_type != &OrderedListType::Numeric {
        format_list(
            elements,
            &format!("ol type=\"{}\"", num_type.to_str()),
            "ol",
        )
    } else {
        format_list(elements, "ol", "ol")
    }
}

fn format_codeblock(lang: &Option<String>, elements: &str) -> String {
    if lang.is_none() || (lang.is_some() && lang.as_ref().unwrap().is_empty()) {
        format!("<pre><code>{}</code></pre>\n\n", &escape(elements, false))
    } else {
        format!(
            "<pre><code class=\"language-{}\">{}</code></pre>\n\n",
            &escape(lang.as_ref().unwrap(), false),
            &escape(elements, false)
        )
    }
}

fn format_blockquote(elements: &[Block]) -> String {
    format!("<blockquote>\n{}</blockquote>\n\n", to_html(elements))
}

fn format_paragraph(elements: &[Span]) -> String {
    format!("<p>{}</p>\n\n", format_spans(elements))
}

fn format_header(elements: &[Span], level: usize) -> String {
    format!(
        "<h{} id='{}'>{}</h{}>\n\n",
        level,
        slugify(elements),
        format_spans(elements),
        level
    )
}
