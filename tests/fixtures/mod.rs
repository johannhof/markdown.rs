use std::path::Path;
use std::fs::File;
use std::io::Read;
use markdown;
use text_diff;

fn compare(name: &str) {
    let html = format!("tests/fixtures/docs-maruku-unittest/{}.html", name);
    let text = format!("tests/fixtures/docs-maruku-unittest/{}.text", name);
    let mut comp = String::new();
    File::open(Path::new(&html)).unwrap().read_to_string(&mut comp);
    let md = Path::new(&text);

    let mut tokens = String::new();
    File::open(md).unwrap().read_to_string(&mut tokens);
    println!("{:?}", markdown::tokenize(&tokens));

    text_diff::assert_diff(&comp, &markdown::file_to_html(md).unwrap(), " ", 0);
}

#[test]
pub fn alt() {
    compare("alt")
}

#[test]
pub fn blank() {
    compare("blank")
}

#[test]
pub fn code() {
    compare("code")
}

#[test]
pub fn easy() {
    compare("easy")
}

#[test]
pub fn one() {
    compare("one")
}

#[test]
pub fn paragraph() {
    compare("paragraph")
}

#[test]
pub fn paragraphs() {
    compare("paragraphs")
}

#[test]
pub fn test() {
    compare("test")
}
