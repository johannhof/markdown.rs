use regex::Regex;

static SETEXT_HEADER_1 : Regex = regex!(r"(^\n)?(\S+) *(\n===+\n)(\n)?");
static SETEXT_HEADER_2 : Regex = regex!(r"(^\n)?(\S+) *(\n---+\n)(\n)?");
static ATX_HEADER : Regex = regex!(r"(^\n)?(#{1,6}\s[^\s]*\n)(\n)?");
static BREAK : Regex = regex!(r"( {2,}\n)");

pub fn normalize(text : &str) -> Box<String> {
    let mut ret = SETEXT_HEADER_1.replace_all(text, "\n$2$3\n");
    ret = SETEXT_HEADER_2.replace_all(ret.as_slice(), "\n$2$3\n");
    ret = ATX_HEADER.replace_all(ret.as_slice(), "\n$2\n");
    ret = BREAK.replace_all(ret.as_slice(), "\n\n  \n\n");
    box ret
}
