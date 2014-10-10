use regex::Regex;

// TODO: i don't like what this file does, get rid of it eventually

static SETEXT_HEADER : Regex = regex!(r"(^\n)?(\S+) *(\n[-=]{3,}+\n)(\n)?");
static ATX_HEADER : Regex = regex!(r"(\n#{1,6}\s.*)[^(?: #+\n)]|(?: #+\n)");

pub fn normalize(text : &str) -> String {
    let mut ret = SETEXT_HEADER.replace_all(text, "\n$2$3\n");
    ret = ATX_HEADER.replace_all(ret.as_slice(), "\n$1\n");
    ret
}

#[test]
fn normalize_setext_header_test() {
    assert_eq!(normalize("wot\nABC\n=======\nwat"),
                box "wot\n\nABC\n=======\n\nwat".to_string());
    assert_eq!(normalize("=======\nABC\n=======\n======="),
                box "=======\n\nABC\n=======\n\n=======".to_string());

    assert_eq!(normalize("wot\nABC\n-------\nwat"),
                box "wot\n\nABC\n-------\n\nwat".to_string());
}

#[test]
fn normalize_atx_header_test() {
    assert_eq!(normalize("### ABC ####\n"),
                box "### ABC\n\n".to_string());
    //assert_eq!(normalize("### ABC####\n"),
                //box "### ABC####\n\n".to_string());
}

