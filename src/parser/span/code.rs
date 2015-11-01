use regex::Regex;
use parser::Span;
use parser::Span::Code;

pub fn parse_code(text: &str) -> Option<(Span, usize)>{
    let code_single = Regex::new(r"^`(?P<text>.+?)`").unwrap();
    let code_double = Regex::new(r"^``(?P<text>.+?)``").unwrap();

    if code_double.is_match(text){
        let caps = code_double.captures(text).unwrap();
        let t = caps.name("text").unwrap();
        return Some((Code(t.to_owned()), t.len() + 4));
    }else if code_single.is_match(text){
        let caps = code_single.captures(text).unwrap();
        let t = caps.name("text").unwrap();
        return Some((Code(t.to_owned()), t.len() + 2));
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
    assert_eq!(
        parse_code("`` testing things test"),
        None
    );
    assert_eq!(
        parse_code("` test"),
        None
    );
}

#[test]
fn no_early_matching() {
    assert_eq!(
        parse_code("were ``testing things`` test"),
        None
    );
    assert_eq!(
        parse_code("were `testing things` test"),
        None
    );
}
