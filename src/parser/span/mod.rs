use parser::Span;
use parser::Span::Text;

mod br;
mod code;
mod emphasis;
mod strong;
mod link;
mod image;
use self::br::parse_break;
use self::code::parse_code;
use self::emphasis::parse_emphasis;
use self::strong::parse_strong;
use self::link::parse_link;
use self::image::parse_image;

pub fn parse_spans(text: &str) -> Vec<Span>{
    let mut tokens = vec![];
    let mut t = String::new();
    let mut i = 0;
    while i < text.len() {
        match parse_span(&text[i .. text.len()]){
            Some((span, consumed_chars)) => {
                if !t.is_empty(){
                    // if this text is on the very left
                    // trim the left whitespace
                    if tokens.is_empty() {
                        t = t.trim_left().to_string()
                    }
                    tokens.push(Text(t));
                }
                tokens.push(span);
                t = String::new();
                i += consumed_chars;
            }
            None => {
                t.push_str(&text[i..i + 1]);
                i += 1;
            }
        }
    }
    if !t.is_empty(){
        // if this text is on the very left
        // trim the left whitespace
        if tokens.is_empty() {
            t = t.trim_left().to_string()
        }
        tokens.push(Text(t));
    }
    tokens
}

fn parse_span(text: &str) -> Option<(Span, usize)>{
    pipe_opt!(
        text
        => parse_code
        => parse_strong
        => parse_emphasis
        => parse_break
        => parse_image
        => parse_link
    )
}

#[cfg(test)]
mod test {
    use parser::Span::{Text, Break, Code, Emphasis, Strong, Link, Image};
    use parser::span::parse_spans;

    #[test]
    fn converts_into_text() {
        assert_eq!(parse_spans("this is a test"), vec![Text("this is a test".to_string())]);
    }

    #[test]
    fn finds_breaks() {
        assert_eq!(parse_spans("this is a test  "), vec![Text("this is a test".to_string()), Break]);
    }

    #[test]
    fn finds_code() {
        assert_eq!(parse_spans("this `is a` test"), vec![Text("this ".to_string()), Code("is a".to_string()), Text(" test".to_string())]);
        assert_eq!(parse_spans("this ``is a`` test"), vec![Text("this ".to_string()), Code("is a".to_string()), Text(" test".to_string())]);
    }

    #[test]
    fn finds_emphasis() {
        assert_eq!(parse_spans("this _is a_ test"), vec![Text("this ".to_string()), Emphasis(vec![Text("is a".to_string())]), Text(" test".to_string())]);
        assert_eq!(parse_spans("this *is a* test"), vec![Text("this ".to_string()), Emphasis(vec![Text("is a".to_string())]), Text(" test".to_string())]);
    }

    #[test]
    fn finds_strong() {
        assert_eq!(parse_spans("this __is a__ test"), vec![Text("this ".to_string()), Strong(vec![Text("is a".to_string())]), Text(" test".to_string())]);
        assert_eq!(parse_spans("this **is a** test"), vec![Text("this ".to_string()), Strong(vec![Text("is a".to_string())]), Text(" test".to_string())]);
    }

    #[test]
    fn finds_link() {
        assert_eq!(
            parse_spans("this is [an example](example.com) test"),
            vec![Text("this is ".to_string()), Link("an example".to_string(), "example.com".to_string(), None), Text(" test".to_string())]
        );
    }

    #[test]
    fn finds_image() {
        assert_eq!(
            parse_spans("this is ![an example](example.com) test"),
            vec![Text("this is ".to_string()), Image("an example".to_string(), "example.com".to_string(), None), Text(" test".to_string())]
        );
    }

    #[test]
    fn finds_everything() {
        assert_eq!(
            parse_spans("some text ![an image](image.com) _emphasis_ __strong__ `teh codez` [a link](example.com)  "),
            vec![
                Text("some text ".to_string()),
                Image("an image".to_string(), "image.com".to_string(), None),
                Text(" ".to_string()),
                Emphasis(vec![Text("emphasis".to_string())]),
                Text(" ".to_string()),
                Strong(vec![Text("strong".to_string())]),
                Text(" ".to_string()),
                Code("teh codez".to_string()),
                Text(" ".to_string()),
                Link("a link".to_string(), "example.com".to_string(), None),
                Break
            ]
        );
    }
}

