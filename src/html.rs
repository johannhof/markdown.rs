use parser::Block;
use parser::Block::{
    Blockquote, CodeBlock, Header, Hr, LinkReference, OrderedList, Paragraph, Raw, UnorderedList,
};
use parser::Span::{Break, Code, Emphasis, Image, Link, Literal, RefLink, Strong, Text};
use parser::{ListItem, OrderedListType, Span};
use regex::Regex;
use std::collections::HashMap;

type LinkReferenceMap<'a> = HashMap<&'a str, (&'a str, Option<&'a str>)>;

// takes a number of elements and returns their collective text as a slug
fn slugify(elements: &[Span], no_spaces: bool) -> String {
    let mut ret = String::new();

    for el in elements {
        let next = match *el {
            Break => "".to_owned(),
            Literal(character) => character.to_string(),
            Text(ref text) => text.trim().to_lowercase(),
            Image(ref text, _, _) | Code(ref text) => text.trim().to_lowercase(),
            RefLink(ref content, _, _)
            | Link(ref content, _, _)
            | Strong(ref content)
            | Emphasis(ref content) => slugify(content, no_spaces),
        };
        if !ret.is_empty() {
            ret.push('_');
        }
        ret.push_str(&next);
    }

    if no_spaces {
        ret = ret.replace(" ", "_");
    }

    ret
}

pub fn to_html(blocks: &[Block]) -> String {
    let mut ret = String::new();
    let mut link_references: LinkReferenceMap = HashMap::new();
    for block in blocks.iter() {
        match block {
            LinkReference(ref id, ref text, ref title) => {
                link_references.insert(id, (text, *title));
            }
            _ => {}
        };
    }
    for block in blocks.iter() {
        let next = match block {
            Header(ref elements, level) => format_header(elements, *level, &link_references),
            Paragraph(ref elements) => format_paragraph(elements, &link_references),
            Blockquote(ref elements) => format_blockquote(elements),
            CodeBlock(ref lang, ref elements) => format_codeblock(lang, elements),
            UnorderedList(ref elements) => format_unordered_list(elements, &link_references),
            OrderedList(ref elements, ref num_type) => {
                format_ordered_list(elements, num_type, &link_references)
            }
            LinkReference(_, _, _) => "".to_owned(),
            Raw(elements) => elements.to_string(),
            Hr => format!("<hr />\n\n"),
        };
        ret.push_str(&next)
    }
    ret = ret.trim().to_owned();
    ret.push('\n');
    ret
}

fn format_spans(elements: &[Span], link_references: &LinkReferenceMap) -> String {
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
                format_spans(content, link_references)
            ),
            Link(ref content, ref url, Some(ref title)) => format!(
                "<a href=\"{}\" title=\"{}\">{}</a>",
                &escape(url, false),
                &escape(title, true),
                format_spans(content, link_references)
            ),
            RefLink(ref content, ref reference, ref raw) => {
                if let Some((ref url, None)) = link_references.get::<str>(reference) {
                    format!(
                        "<a href=\"{}\">{}</a>",
                        &escape(url, false),
                        format_spans(content, link_references)
                    )
                } else if let Some((ref url, Some(ref title))) =
                    link_references.get::<str>(reference)
                {
                    format!(
                        "<a href=\"{}\" title=\"{}\">{}</a>",
                        &escape(url, false),
                        &escape(title, true),
                        format_spans(content, link_references)
                    )
                } else if let Some((ref url, None)) =
                    link_references.get::<str>(&slugify(content, false))
                {
                    format!(
                        "<a href=\"{}\">{}</a>",
                        &escape(url, false),
                        format_spans(content, link_references)
                    )
                } else if let Some((ref url, Some(ref title))) =
                    link_references.get::<str>(&slugify(content, false))
                {
                    format!(
                        "<a href=\"{}\" title=\"{}\">{}</a>",
                        &escape(url, false),
                        &escape(title, true),
                        format_spans(content, link_references)
                    )
                } else {
                    raw.to_string()
                }
            }
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
            Emphasis(ref content) => format!("<em>{}</em>", format_spans(content, link_references)),
            Strong(ref content) => format!(
                "<strong>{}</strong>",
                format_spans(content, link_references)
            ),
        };
        ret.push_str(&next)
    }
    ret
}

fn escape(text: &str, replace_entities: bool) -> String {
    lazy_static! {
        static ref AMPERSAND: Regex = Regex::new(r"&amp;(?P<x>\S+;)").unwrap();
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

fn format_list(
    elements: &[ListItem],
    start_tag: &str,
    end_tag: &str,
    link_references: &LinkReferenceMap,
) -> String {
    let mut ret = String::new();
    for list_item in elements {
        let mut content = String::new();
        match *list_item {
            ListItem::Simple(ref els) => content.push_str(&format_spans(els, link_references)),
            ListItem::Paragraph(ref paragraphs) => {
                content.push_str(&format!("\n{}", to_html(paragraphs)))
            }
        }

        ret.push_str(&format!("\n<li>{}</li>\n", content))
    }
    format!("<{}>{}</{}>\n\n", start_tag, ret, end_tag)
}

fn format_unordered_list(elements: &[ListItem], link_references: &LinkReferenceMap) -> String {
    format_list(elements, "ul", "ul", link_references)
}

fn format_ordered_list(
    elements: &[ListItem],
    num_type: &OrderedListType,
    link_references: &LinkReferenceMap,
) -> String {
    if num_type != &OrderedListType::Numeric {
        format_list(
            elements,
            &format!("ol type=\"{}\"", num_type.to_html_type()),
            "ol",
            link_references,
        )
    } else {
        format_list(elements, "ol", "ol", link_references)
    }
}

fn format_codeblock(lang: &Option<&str>, elements: &[&str]) -> String {
    let code = elements.iter()
        .map(|s| s.to_owned())
        .collect::<Vec<_>>()
        .join("\n");
    if lang.is_none() || (lang.is_some() && lang.as_ref().unwrap().is_empty()) {
        format!("<pre><code>{}</code></pre>\n\n", &escape(&code, false))
    } else {
        format!(
            "<pre><code class=\"language-{}\">{}</code></pre>\n\n",
            &escape(lang.as_ref().unwrap(), false),
            &escape(&code, false)
        )
    }
}

fn format_blockquote(elements: &[Block]) -> String {
    format!("<blockquote>\n{}</blockquote>\n\n", to_html(elements))
}

fn format_paragraph(elements: &[Span], link_references: &LinkReferenceMap) -> String {
    format!("<p>{}</p>\n\n", format_spans(elements, link_references))
}

fn format_header(elements: &[Span], level: usize, link_references: &LinkReferenceMap) -> String {
    format!(
        "<h{} id='{}'>{}</h{}>\n\n",
        level,
        slugify(elements, true),
        format_spans(elements, link_references),
        level
    )
}
