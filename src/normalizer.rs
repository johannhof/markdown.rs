use regex::Regex;

static SETEXT_HEADER_1 : Regex = regex!(r"(^\n)?(\S+) *(\n===+\n)(\n)?");
static SETEXT_HEADER_2 : Regex = regex!(r"(^\n)?(\S+) *(\n---+\n)(\n)?");
static ATX_HEADER : Regex = regex!(r"(\n#{1,6}\s.*)[^(?: #+\n)]|(?: #+\n)");
static BLOCKQUOTES : Regex = regex!(r"([^\n]\n)(>\s)");
static EMPTY_BLOCKQUOTES : Regex = regex!(r">\n");

pub fn normalize(text : &str) -> Box<String> {
    let mut ret = SETEXT_HEADER_1.replace_all(text, "\n$2$3\n");
    ret = SETEXT_HEADER_2.replace_all(ret.as_slice(), "\n$2$3\n");
    ret = ATX_HEADER.replace_all(ret.as_slice(), "\n$1\n");
    ret = EMPTY_BLOCKQUOTES.replace_all(ret.as_slice(), "");
    ret = BLOCKQUOTES.replace_all(ret.as_slice(), "$1");
    box ret
}

#[test]
fn normalize_test() {
    assert_eq!(normalize("### ABC ####\n"),
                box "### ABC\n\n".to_string());
    assert_eq!(normalize("### ABC####\n"),
                box "### ABC\n\n".to_string());
}

