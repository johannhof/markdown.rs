use regex::Regex;

// TODO: i don't like what this file does, get rid of it eventually

static SETEXT_HEADER : Regex = regex!(r"\n(\S+) *(\n[-=]{3,}+)\n?");
static ATX_HEADER : Regex = regex!(r"(?m)^(#{1,6}\s.*?)(?:\s#+\s*?)?\n");

pub fn normalize(text : &str) -> String {
    let mut ret = SETEXT_HEADER.replace_all(text, "\n\n$1$2\n\n");
    ret = ATX_HEADER.replace_all(ret.as_slice(), "\n$1\n\n");
    ret
}

#[test]
fn normalize_setext_header_test() {
    assert_eq!(normalize("wot\nABC\n=======\nwat"),
                "wot\n\nABC\n=======\n\nwat".to_string());
    assert_eq!(normalize("=======\nABC\n=======\n======="),
                "=======\n\nABC\n=======\n\n=======".to_string());

    assert_eq!(normalize("wot\nABC\n-------\nwat"),
                "wot\n\nABC\n-------\n\nwat".to_string());
}

#[test]
fn normalize_atx_header_test() {
    assert_eq!(normalize("### ABC ####\n"),
                "\n### ABC\n\n".to_string());
    assert_eq!(normalize("### ABC####\n"),
                "\n### ABC####\n\n".to_string());
    assert_eq!(normalize("### ABC#### #\n"),
                "\n### ABC####\n\n".to_string());
}

