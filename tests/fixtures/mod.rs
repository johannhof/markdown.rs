use std::path::Path;
use std::fs::File;
use std::io::Read;
use markdown;
use text_diff;

fn compare(html: &str, text: &str) {
    let mut comp = String::new();
    File::open(Path::new(html)).unwrap().read_to_string(&mut comp);
    let md = Path::new(text);
    text_diff::assert_diff(&comp, &markdown::file_to_html(md).unwrap(), " ", 0);
}

#[test]
pub fn alt() {
    compare("tests/fixtures/docs-maruku-unittest/alt.html", "tests/fixtures/docs-maruku-unittest/alt.text")
}

#[test]
pub fn blank() {
    compare("tests/fixtures/docs-maruku-unittest/blank.html", "tests/fixtures/docs-maruku-unittest/blank.text")
}

