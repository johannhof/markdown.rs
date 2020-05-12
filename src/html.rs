use parser::Block;
use parser::Block::{
    Blockquote, CodeBlock, Header, Hr, OrderedList, Paragraph, Raw, UnorderedList, LinkReference
};
use parser::Span::{Break, Code, Emphasis, Image, Link, ReferenceLink, Strong, Text};
use parser::{ListItem, OrderedListType, Span};

// takes a number of elements and returns their collective text as a slug
fn slugify(elements: &[Span]) -> String {
    let mut ret = String::new();

    for el in elements {
        let next = match *el {
            Break => "".to_owned(),
            Text(ref text) | Link(ref text, _, _) | ReferenceLink(ref text, _) | Image(ref text, _, _) | Code(ref text) => {
                text.trim().replace(" ", "_").to_lowercase().to_owned()
            }
            Strong(ref content) | Emphasis(ref content) => slugify(content),
        };
        if !ret.is_empty() {
            ret.push('_');
        }
        ret.push_str(&next);
    }

    ret
}

pub fn to_html(blocks: &[Block]) -> String {
    let mut references: Vec<(&str, &str, &Option<String>)> = vec![];
    for block in blocks.iter() {
        match block {
            LinkReference(ref reference, ref link, ref title) => references.push((reference, link, title)),
            _ => ()
        }
    }

    let mut ret = String::new();
    for block in blocks.iter() {
        let next = match *block {
            Header(ref elements, level) => format_header(&references, elements, level),
            Paragraph(ref elements) => format_paragraph(&references, elements),
            Blockquote(ref elements) => format_blockquote(elements),
            CodeBlock(ref lang, ref elements) => format_codeblock(lang, elements),
            UnorderedList(ref elements) => format_unordered_list(&references, elements),
            OrderedList(ref elements, ref num_type) => format_ordered_list(&references, elements, num_type),
            LinkReference(_, _, _) => String::new(),
            Raw(ref elements) => elements.to_owned(),
            Hr => format!("<hr>"),
        };
        ret.push_str(&next)
    }

    ret = ret.trim().to_owned();
    ret.push('\n');
    ret
}

fn format_spans(references: &[(&str, &str, &Option<String>)], elements: &[Span]) -> String {
    let mut ret = String::new();
    for element in elements.iter() {
        let next = match *element {
            Break => format!("<br />"),
            Text(ref text) => format!("{}", &escape(text)),
            Code(ref text) => format!("<code>{}</code>", &escape(text)),
            Link(ref text, ref url, None) => {
                format!("<a href='{}'>{}</a>", &escape(url), &escape(text))
            }
            Link(ref text, ref url, Some(ref title)) => format!(
                "<a href='{}' title='{}'>{}</a>",
                &escape(url),
                &escape(title),
                &escape(text)
            ),
            ReferenceLink(ref text, ref reference) => {
                let mut matched = String::new();
                for (refer, url, title_op) in references.iter() {
                    if refer == reference {
                        if let Some(title) = title_op {
                            matched = format!(
                                "<a href='{}' title='{}'>{}</a>",
                                &escape(url),
                                &escape(title),
                                &escape(text)
                            );
                        } else {
                            matched = format!(
                                "<a href='{}'>{}</a>",
                                &escape(url),
                                &escape(text)
                            );
                        }
                    }
                }

                if matched.is_empty() {
                    format!("[{}][{}]", text, reference)
                } else {
                    matched
                }
            },
            Image(ref text, ref url, None) => {
                format!("<img src='{}' alt='{}' />", &escape(url), &escape(text))
            }
            Image(ref text, ref url, Some(ref title)) => format!(
                "<img src='{}' title='{}' alt='{}' />",
                &escape(url),
                &escape(title),
                &escape(text)
            ),
            Emphasis(ref content) => format!("<em>{}</em>", format_spans(references, content)),
            Strong(ref content) => format!("<strong>{}</strong>", format_spans(references, content)),
        };
        ret.push_str(&next)
    }
    ret
}

fn escape(text: &str) -> String {
    text.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace("\"", "&quot;")
        .replace("'", "&#8217;")
        .replace(">", "&gt;")
}

fn format_list(references: &[(&str, &str, &Option<String>)], elements: &[ListItem], start_tag: &str, end_tag: &str) -> String {
    let mut ret = String::new();
    for list_item in elements {
        let mut content = String::new();
        match *list_item {
            ListItem::Simple(ref els) => content.push_str(&format_spans(references, els)),
            ListItem::Paragraph(ref paragraphs) => {
                content.push_str(&format!("\n{}", to_html(paragraphs)))
            }
        }

        ret.push_str(&format!("\n<li>{}</li>\n", content))
    }
    format!("<{}>{}</{}>\n\n", start_tag, ret, end_tag)
}

fn format_unordered_list(references: &[(&str, &str, &Option<String>)], elements: &[ListItem]) -> String {
    format_list(references, elements, "ul", "ul")
}

fn format_ordered_list(references: &[(&str, &str, &Option<String>)], elements: &[ListItem], num_type: &OrderedListType) -> String {
    format_list(references, elements, &format!("ol type=\"{}\"", num_type.0), "ol")
}

fn format_codeblock(lang: &Option<String>, elements: &str) -> String {
    if lang.is_none() || (lang.is_some() && lang.as_ref().unwrap().is_empty()) {
        format!("<pre><code>{}</code></pre>\n\n", &escape(elements))
    } else {
        format!(
            "<pre><code class=\"language-{}\">{}</code></pre>\n\n",
            &escape(lang.as_ref().unwrap()),
            &escape(elements)
        )
    }
}

fn format_blockquote(elements: &[Block]) -> String {
    format!("<blockquote>\n{}</blockquote>\n\n", to_html(elements))
}

fn format_paragraph(references: &[(&str, &str, &Option<String>)], elements: &[Span]) -> String {
    format!("<p>{}</p>\n\n", format_spans(references, elements))
}

fn format_header(references: &[(&str, &str, &Option<String>)], elements: &[Span], level: usize) -> String {
    format!(
        "<h{} id='{}'>{}</h{}>\n\n",
        level,
        slugify(elements),
        format_spans(references, elements),
        level
    )
}
