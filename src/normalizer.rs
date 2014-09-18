use regex::Regex;

static SETEXT_HEADER_1 : Regex = regex!(r"(^\n)?(.+\n===+\n)(\n)?");
static SETEXT_HEADER_2 : Regex = regex!(r"(^\n)?(.+\n---+\n)(\n)?");
static ATX_HEADER : Regex = regex!(r"(^\n)?(#{1,6}\s[^\s]*\n)(\n)?");

pub fn normalize(text : &str) -> Box<String> {
    let mut ret = SETEXT_HEADER_1.replace_all(text, "\n$2\n");
    ret = SETEXT_HEADER_2.replace_all(ret.as_slice(), "\n$2\n");
    ret = ATX_HEADER.replace_all(ret.as_slice(), "\n$2\n");
    box ret
}
