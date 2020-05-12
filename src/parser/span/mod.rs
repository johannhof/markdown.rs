use parser::Span;
use parser::Span::Text;

mod br;
mod code;
mod emphasis;
mod image;
mod link;
mod reference_link;
mod strong;
use self::br::parse_break;
use self::code::parse_code;
use self::emphasis::parse_emphasis;
use self::image::parse_image;
use self::link::parse_link;
use self::reference_link::parse_reference_link;
use self::strong::parse_strong;

pub fn parse_spans(text: &str) -> Vec<Span> {
    let mut tokens = vec![];
    let mut t = String::new();
    let mut i = 0;
    while i < text.len() {
        match parse_span(&text[i..text.len()]) {
            Some((span, consumed_chars)) => {
                if !t.is_empty() {
                    // if this text is on the very left
                    // trim the left whitespace
                    if tokens.is_empty() {
                        t = t.trim_start().to_owned()
                    }
                    tokens.push(Text(t));
                }
                tokens.push(span);
                t = String::new();
                i += consumed_chars;
            }
            None => {
                let mut e = i + 1;
                while !text.is_char_boundary(e) {
                    e += 1;
                }

                t.push_str(&text[i..e]);
                i += e - i;
            }
        }
    }
    if !t.is_empty() {
        // if this text is on the very left
        // trim the left whitespace
        if tokens.is_empty() {
            t = t.trim_start().to_owned();
        }
        // we're at the very end of this line,
        // trim trailing whitespace
        t = t.trim_end().to_owned();
        tokens.push(Text(t));
    }
    tokens
}

fn parse_span(text: &str) -> Option<(Span, usize)> {
    pipe_opt!(
    text
    => parse_code
    => parse_strong
    => parse_emphasis
    => parse_break
    => parse_image
    => parse_link
    => parse_reference_link
    )
}

#[cfg(test)]
mod test {
    use parser::span::parse_spans;
    use parser::Span::{Break, Code, Emphasis, Image, Link, ReferenceLink, Strong, Text};
    use std::str;

    #[test]
    fn converts_into_text() {
        assert_eq!(
            parse_spans("this is a test"),
            vec![Text("this is a test".to_owned())]
        );
    }

    #[test]
    fn finds_breaks() {
        assert_eq!(
            parse_spans("this is a test  "),
            vec![Text("this is a test".to_owned()), Break]
        );
    }

    #[test]
    fn finds_code() {
        assert_eq!(
            parse_spans("this `is a` test"),
            vec![
                Text("this ".to_owned()),
                Code("is a".to_owned()),
                Text(" test".to_owned())
            ]
        );
        assert_eq!(
            parse_spans("this ``is a`` test"),
            vec![
                Text("this ".to_owned()),
                Code("is a".to_owned()),
                Text(" test".to_owned())
            ]
        );
    }

    #[test]
    fn finds_emphasis() {
        assert_eq!(
            parse_spans("this _is a_ test"),
            vec![
                Text("this ".to_owned()),
                Emphasis(vec![Text("is a".to_owned())]),
                Text(" test".to_owned())
            ]
        );
        assert_eq!(
            parse_spans("this *is a* test"),
            vec![
                Text("this ".to_owned()),
                Emphasis(vec![Text("is a".to_owned())]),
                Text(" test".to_owned())
            ]
        );
    }

    #[test]
    fn finds_strong() {
        assert_eq!(
            parse_spans("this __is a__ test"),
            vec![
                Text("this ".to_owned()),
                Strong(vec![Text("is a".to_owned())]),
                Text(" test".to_owned())
            ]
        );
        assert_eq!(
            parse_spans("this **is a** test"),
            vec![
                Text("this ".to_owned()),
                Strong(vec![Text("is a".to_owned())]),
                Text(" test".to_owned())
            ]
        );
    }

    #[test]
    fn finds_link() {
        assert_eq!(
            parse_spans("this is [an example](example.com) test"),
            vec![
                Text("this is ".to_owned()),
                Link("an example".to_owned(), "example.com".to_owned(), None),
                Text(" test".to_owned())
            ]
        );
    }

    #[test]
    fn finds_reference_link() {
        assert_eq!(
            parse_spans("this is [an example][ref] test"),
            vec![
                Text("this is ".to_owned()),
                ReferenceLink("an example".to_owned(), "ref".to_owned()),
                Text(" test".to_owned())
            ]
        );
    }

    #[test]
    fn finds_image() {
        assert_eq!(
            parse_spans("this is ![an example](example.com) test"),
            vec![
                Text("this is ".to_owned()),
                Image("an example".to_owned(), "example.com".to_owned(), None),
                Text(" test".to_owned())
            ]
        );
    }

    #[test]
    fn finds_everything() {
        assert_eq!(
            parse_spans("some text ![an image](image.com) _emphasis_ __strong__ `teh codez` [a link](example.com) [ref link][ref]  "),
            vec![
            Text("some text ".to_owned()),
            Image("an image".to_owned(), "image.com".to_owned(), None),
            Text(" ".to_owned()),
            Emphasis(vec![Text("emphasis".to_owned())]),
            Text(" ".to_owned()),
            Strong(vec![Text("strong".to_owned())]),
            Text(" ".to_owned()),
            Code("teh codez".to_owned()),
            Text(" ".to_owned()),
            Link("a link".to_owned(), "example.com".to_owned(), None),
            Text(" ".to_owned()),
            ReferenceLink("ref link".to_owned(), "ref".to_owned()),
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
