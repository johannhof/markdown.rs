use parser::Span;
use parser::Span::Code;
use regex::Regex;

pub fn parse_code(text: &str) -> Option<(Span, usize)> {
    lazy_static! {
        static ref CODE: Regex =
            Regex::new(r"^``(?P<double_text>.+?)``|^`(?P<single_text>.+?)`").unwrap();
    }

    if CODE.is_match(text) {
        let caps = CODE.captures(text).expect("is_match returned true");
        return match (caps.name("double_text"), caps.name("single_text")) {
            (Some(m), _) => {
                let t = m.as_str();
                Some((Code(t.to_owned()), t.len() + 4))
            }
            (None, Some(m)) => {
                let t = m.as_str();
                Some((Code(t.to_owned()), t.len() + 2))
            }
            _ => unreachable!("CODE.is_match returned true"),
        };
    }
    None
}

#[test]
fn finds_code() {
    assert_eq!(
        parse_code("`testing things` test"),
        Some((Code("testing things".to_owned()), 16))
    );

    assert_eq!(
        parse_code("``testing things`` test"),
        Some((Code("testing things".to_owned()), 18))
    );

    assert_eq!(
        parse_code("``testing things`` things`` test"),
        Some((Code("testing things".to_owned()), 18))
    );

    assert_eq!(
        parse_code("`w` testing things test"),
        Some((Code("w".to_owned()), 3))
    );

    assert_eq!(
        parse_code("`w`` testing things test"),
        Some((Code("w".to_owned()), 3))
    );

    assert_eq!(
        parse_code("``w`` testing things test"),
        Some((Code("w".to_owned()), 5))
    );

    assert_eq!(
        parse_code("``w``` testing things test"),
        Some((Code("w".to_owned()), 5))
    );
}

#[test]
fn no_false_positives() {
    assert_eq!(parse_code("`` testing things test"), None);
    assert_eq!(parse_code("` test"), None);
}

#[test]
fn no_early_matching() {
    assert_eq!(parse_code("were ``testing things`` test"), None);
    assert_eq!(parse_code("were `testing things` test"), None);
}
