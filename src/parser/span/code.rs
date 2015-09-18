use regex::Regex;
use parser::Span;
use parser::Span::Code;

pub fn parse_code(text: &str) -> Option<(Span, usize)>{
    let CODE_SINGLE = Regex::new(r"^`(?P<text>.+?)`").unwrap();
    let CODE_DOUBLE = Regex::new(r"^``(?P<text>.+?)``").unwrap();

    if CODE_DOUBLE.is_match(text){
        let caps = CODE_DOUBLE.captures(text).unwrap();
        let t = caps.name("text").unwrap();
        return Some((Code(t.to_string()), t.len() + 4));
    }else if CODE_SINGLE.is_match(text){
        let caps = CODE_SINGLE.captures(text).unwrap();
        let t = caps.name("text").unwrap();
        return Some((Code(t.to_string()), t.len() + 2));
    }
    return None;
}

#[test]
fn finds_code() {
    assert_eq!(
        parse_code("`testing things` test"),
        Some((Code("testing things".to_string()), 16))
    );

    assert_eq!(
        parse_code("``testing things`` test"),
        Some((Code("testing things".to_string()), 18))
    );

    assert_eq!(
        parse_code("``testing things`` things`` test"),
        Some((Code("testing things".to_string()), 18))
    );

    assert_eq!(
        parse_code("`w` testing things test"),
        Some((Code("w".to_string()), 3))
    );

    assert_eq!(
        parse_code("`w`` testing things test"),
        Some((Code("w".to_string()), 3))
    );

    assert_eq!(
        parse_code("``w`` testing things test"),
        Some((Code("w".to_string()), 5))
    );

    assert_eq!(
        parse_code("``w``` testing things test"),
        Some((Code("w".to_string()), 5))
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
