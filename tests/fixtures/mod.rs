use std::path::Path;
use std::fs::File;
use std::io::Read;
use markdown;
use difference;

fn compare(name: &str) {
    let html = format!("tests/fixtures/files/{}.html", name);
    let text = format!("tests/fixtures/files/{}.text", name);
    let mut comp = String::new();
    File::open(Path::new(&html)).unwrap().read_to_string(&mut comp).unwrap();
    let md = Path::new(&text);

    let mut tokens = String::new();
    File::open(md).unwrap().read_to_string(&mut tokens).unwrap();
    println!("{:?} -> {:?}", tokens, markdown::tokenize(&tokens));

    difference::assert_diff(&comp, &markdown::file_to_html(md).unwrap(), " ", 0);
}

fn roundtrip(name: &str) {
    let html = format!("tests/fixtures/files/{}.html", name);
    let text = format!("tests/fixtures/files/{}.text", name);
    let mut comp = String::new();
    File::open(Path::new(&html)).unwrap().read_to_string(&mut comp).unwrap();
    let md = Path::new(&text);

    let mut tokens = String::new();
    File::open(md).unwrap().read_to_string(&mut tokens).unwrap();

    let v = markdown::tokenize(&tokens);
    println!("{:?}", v);
    let out = markdown::generate_markdown(v);

    println!("BEGIN\n{}\nEND", out);

    difference::assert_diff(&comp, &markdown::to_html(&out), " ", 0);
}

#[test]
pub fn alt() {
    compare("alt")
}

#[test]
pub fn rt_alt() {
    roundtrip("alt")
}

#[test]
pub fn blank() {
    compare("blank")
}

#[test]
pub fn rt_blank() {
    roundtrip("blank")
}

#[test]
pub fn blanks_in_code() {
    compare("blanks_in_code")
}

#[test]
pub fn rt_blanks_in_code() {
    roundtrip("blanks_in_code")
}

#[test]
pub fn code() {
    compare("code")
}

#[test]
pub fn rt_code() {
    roundtrip("code")
}

#[test]
pub fn code2() {
    compare("code2")
}

#[test]
pub fn rt_code2() {
    roundtrip("code2")
}

#[test]
pub fn code3() {
    compare("code3")
}

#[test]
pub fn rt_code3() {
    roundtrip("code3")
}

#[test]
pub fn easy() {
    compare("easy")
}

#[test]
pub fn rt_easy() {
    roundtrip("easy")
}

#[test]
pub fn entities() {
    compare("entities")
}

#[test]
pub fn rt_entities() {
    roundtrip("entities")
}

#[test]
pub fn headers() {
    compare("headers")
}

#[test]
pub fn rt_headers() {
    roundtrip("headers")
}

#[test]
pub fn list1() {
    compare("list1")
}

#[test]
pub fn rt_list1() {
    roundtrip("list1")
}

#[test]
pub fn list2() {
    compare("list2")
}

#[test]
pub fn rt_list2() {
    roundtrip("list2")
}

#[test]
pub fn list3() {
    compare("list3")
}

#[test]
pub fn rt_list3() {
    roundtrip("list3")
}

#[test]
pub fn lists() {
    compare("lists")
}

#[test]
pub fn rt_lists() {
    roundtrip("lists")
}

#[test]
pub fn lists8() {
    compare("lists8")
}

#[test]
pub fn rt_lists8() {
    roundtrip("lists8")
}

#[test]
pub fn numbers() {
    compare("numbers")
}

#[test]
pub fn rt_numbers() {
    roundtrip("numbers")
}

#[test]
pub fn one() {
    compare("one")
}

#[test]
pub fn rt_one() {
    roundtrip("one")
}

#[test]
pub fn olist() {
    compare("olist")
}

//#[test]
//pub fn rt_olist() {
    //roundtrip("olist")
//}

#[test]
pub fn paragraph() {
    compare("paragraph")
}

#[test]
pub fn rt_paragraph() {
    roundtrip("paragraph")
}

#[test]
pub fn paragraphs() {
    compare("paragraphs")
}

#[test]
pub fn rt_paragraphs() {
    roundtrip("paragraphs")
}

#[test]
pub fn test() {
    compare("test")
}

#[test]
pub fn rt_test() {
    roundtrip("test")
}

#[test]
pub fn utf8() {
    compare("utf8")
}

#[test]
pub fn rt_utf8() {
    roundtrip("utf8")
}

#[test]
pub fn wrapping() {
    compare("wrapping")
}

#[test]
pub fn rt_wrapping() {
    roundtrip("wrapping")
}
