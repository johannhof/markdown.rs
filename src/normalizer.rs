use regex::Regex;

// TODO: i don't like what this file does, get rid of it eventually

static SETEXT_HEADER : Regex = regex!(r"\n(\S+) *(\n[-=]{3,}+)\n?");
static ATX_HEADER : Regex = regex!(r"(?m)^(#{1,6}\s.*?)(?:\s#+\s*?)?\n");
static CODE_BLOCK : Regex = regex!(r"(( {4}.*\n)+)");

pub fn normalize(text : &str) -> String {
    let mut ret = "\n".to_string();
    ret.push_str(text);
    ret = SETEXT_HEADER.replace_all(&ret[], "\n\n$1$2\n\n");
    ret = ATX_HEADER.replace_all(&ret[], "\n$1\n\n");
    ret = CODE_BLOCK.replace_all(&ret[], "\n$1\n\n");
    ret
}

#[test]
fn normalize_setext_header_test() {
    assert_eq!(normalize("wot\nABC\n=======\nwat"),
                "\nwot\n\nABC\n=======\n\nwat".to_string());
    assert_eq!(normalize("=======\nABC\n=======\n======="),
                "\n=======\n\nABC\n=======\n\n=======".to_string());

    assert_eq!(normalize("wot\nABC\n-------\nwat"),
                "\nwot\n\nABC\n-------\n\nwat".to_string());
}

#[test]
fn normalize_atx_header_test() {
    assert_eq!(normalize("### ABC ####\n"),
                "\n\n### ABC\n\n".to_string());
    assert_eq!(normalize("### ABC####\n"),
                "\n\n### ABC####\n\n".to_string());
    assert_eq!(normalize("### ABC#### #\n"),
                "\n\n### ABC####\n\n".to_string());
    assert_eq!(normalize("wat\n### ABC#### #\n"),
                "\nwat\n\n### ABC####\n\n".to_string());
}

