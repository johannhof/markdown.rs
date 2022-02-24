use parser::Span;
use parser::Span::{Literal, Text};
use std::borrow::Cow;

mod code;
mod emphasis;
mod image;
mod link;
mod strong;
use self::code::parse_code;
use self::emphasis::parse_emphasis;
use self::image::parse_image;
use self::link::parse_link;
use self::strong::parse_strong;

enum SpanOrSpecial<'a> {
    Span(Span<'a>),
    SpanWithOffset(usize, Span<'a>)
}

impl<'a> From<Span<'a>> for SpanOrSpecial<'a> {
    fn from(other: Span<'a>) -> Self {
        SpanOrSpecial::Span(other)
    }
}

pub fn parse_spans(text: &str) -> Vec<Span> {
    let mut buf = Vec::new();
    parse_spans_with_buffer(text, &mut buf);
    buf
}

pub fn parse_spans_with_buffer<'a>(text: &'a str, tokens: &mut Vec<Span<'a>>) {
    let mut i = 0;
    let mut text_token_start_index = 0;
    while i < text.len() {
        match parse_span(&text[i..text.len()]) {
            Some((span_or_special, consumed_chars)) => {
                let (offset, span) = match span_or_special {
                    SpanOrSpecial::Span(s) => (0, s),
                    SpanOrSpecial::SpanWithOffset(o, s) => (o, s)
                };
                let t = &text[text_token_start_index..i + offset];
                if !t.is_empty() {
                    // if this text is on the very left
                    // trim the left whitespace
                    if tokens.is_empty() {
                        tokens.push(Text(Cow::Borrowed(t.trim_start())));
                    } else {
                        tokens.push(Text(Cow::Borrowed(t)));
                    }
                }

                tokens.push(span);
                i += consumed_chars;
                text_token_start_index = i;
            }
            None => {
                i += 1;
                while !text.is_char_boundary(i) {
                    i += 1;
                }
            }
        }
    }
    let mut t = &text[text_token_start_index..];
    if !t.is_empty() {
        // if this text is on the very left
        // trim the left whitespace
        if tokens.is_empty() {
            t = t.trim_start();
        }
        // we're at the very end of this line,
        // trim trailing whitespace
        t = t.trim_end();
        tokens.push(Text(Cow::Borrowed(t)));
    }
}

fn parse_escape(text: &str) -> Option<(Span, usize)> {
    let mut chars = text.chars();
    if let Some('\\') = chars.next() {
        return match chars.next() {
            Some(x @ '\\') | Some(x @ '`') | Some(x @ '*') | Some(x @ '_') | Some(x @ '{')
            | Some(x @ '}') | Some(x @ '[') | Some(x @ ']') | Some(x @ '(') | Some(x @ ')')
            | Some(x @ '#') | Some(x @ '+') | Some(x @ '-') | Some(x @ '.') | Some(x @ '!') => {
                Some((Literal(x), 2))
            }
            _ => None,
        };
    }
    None
}

fn parse_span(text: &str) -> Option<(SpanOrSpecial, usize)> {
    let mut chars = text.chars();
    match (chars.next(), chars.next()) {
        (Some('\\'), _) => parse_escape(text).map(|(a, b)| (a.into(), b)),
        (Some('`'), _) => parse_code(text).map(|(a, b)| (a.into(), b)),
        (Some('*'), Some('*')) |
        (Some('_'), Some('_')) => parse_strong(text).map(|(a, b)| (a.into(), b)),
        (Some('*'), _) |
        (Some('_'), _) => parse_emphasis(text).map(|(a, b)| (a.into(), b)),
        (Some(' '), Some(' ')) if text.len() == 2 => Some((Span::Break.into(), 2)),
        (Some('!'), Some('[')) => parse_image(text).map(|(a, b)| (a.into(), b)),
        (Some('['), _) => parse_link(text).map(|(a, b)| (a.into(), b)),
        (Some(c1), Some(x @ '*')) |
        (Some(c1), Some(x @ '_')) => {
            if c1.is_whitespace() && chars.next()?.is_whitespace() {
                Some((SpanOrSpecial::SpanWithOffset(1, Span::Literal(x)), 2))
            } else {
                None
            }
        }
        _ => None
    }
}

#[cfg(test)]
mod test {
    use parser::span::parse_spans;
    use parser::Span::{Break, Code, Emphasis, Image, Link, Literal, Strong, Text};
    use std::str;

    #[test]
    fn converts_into_text() {
        assert_eq!(
            parse_spans("this is a test"),
            vec![Text("this is a test".into())]
        );
    }

    #[test]
    fn finds_escapes() {
        assert_eq!(parse_spans(r"\*"), vec![Literal('*')]);
    }

    #[test]
    fn finds_breaks() {
        assert_eq!(
            parse_spans("this is a test  "),
            vec![Text("this is a test".into()), Break]
        );
    }

    #[test]
    fn finds_code() {
        assert_eq!(
            parse_spans("this `is a` test"),
            vec![
                Text("this ".into()),
                Code("is a".to_owned()),
                Text(" test".into())
            ]
        );
        assert_eq!(
            parse_spans("this ``is a`` test"),
            vec![
                Text("this ".into()),
                Code("is a".to_owned()),
                Text(" test".into())
            ]
        );
    }

    #[test]
    fn finds_emphasis() {
        assert_eq!(
            parse_spans("this _is a_ test"),
            vec![
                Text("this ".into()),
                Emphasis(vec![Text("is a".into())]),
                Text(" test".into())
            ]
        );
        assert_eq!(
            parse_spans("this *is a* test"),
            vec![
                Text("this ".into()),
                Emphasis(vec![Text("is a".into())]),
                Text(" test".into())
            ]
        );
    }

    #[test]
    fn finds_strong() {
        assert_eq!(
            parse_spans("this __is a__ test"),
            vec![
                Text("this ".into()),
                Strong(vec![Text("is a".into())]),
                Text(" test".into())
            ]
        );
        assert_eq!(
            parse_spans("this **is a** test"),
            vec![
                Text("this ".into()),
                Strong(vec![Text("is a".into())]),
                Text(" test".into())
            ]
        );
    }

    #[test]
    fn finds_link() {
        assert_eq!(
            parse_spans("this is [an example](example.com) test"),
            vec![
                Text("this is ".into()),
                Link(
                    vec![Text("an example".into())],
                    "example.com".to_owned(),
                    None
                ),
                Text(" test".into())
            ]
        );
    }

    #[test]
    fn finds_image() {
        assert_eq!(
            parse_spans("this is ![an example](example.com) test"),
            vec![
                Text("this is ".into()),
                Image("an example".to_owned(), "example.com".to_owned(), None),
                Text(" test".into())
            ]
        );
    }

    #[test]
    fn finds_everything() {
        assert_eq!(
            parse_spans("some text ![an image](image.com) _emphasis_ __strong__ `teh codez` [a link](example.com)  "),
            vec![
            Text("some text ".into()),
            Image("an image".to_owned(), "image.com".to_owned(), None),
            Text(" ".into()),
            Emphasis(vec![Text("emphasis".into())]),
            Text(" ".into()),
            Strong(vec![Text("strong".into())]),
            Text(" ".into()),
            Code("teh codez".to_owned()),
            Text(" ".into()),
            Link(vec![Text("a link".into())], "example.com".to_owned(), None),
            Break
            ]
            );
    }

    #[test]
    fn properly_consumes_multibyte_utf8() {
        let test_phrase = str::from_utf8(b"This shouldn\xE2\x80\x99t panic").unwrap();
        let _ = parse_spans(&test_phrase);
    }
}
