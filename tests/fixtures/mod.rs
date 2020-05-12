use difference;
use markdown;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn compare_fn(html: String, text: String) {
    let mut comp = String::new();
    File::open(Path::new(&html))
        .unwrap()
        .read_to_string(&mut comp)
        .unwrap();
    let md = Path::new(&text);

    let mut tokens = String::new();
    File::open(md).unwrap().read_to_string(&mut tokens).unwrap();
    println!("{:?}", markdown::tokenize(&tokens));
    let generated = markdown::file_to_html(md).unwrap();
    println!("{}", generated);

    difference::assert_diff(&comp, &generated, " ", 0);
}

fn compare(name: &str) {
    compare_fn(
        format!("tests/fixtures/docs-maruku-unittest/{}.html", name),
        format!("tests/fixtures/docs-maruku-unittest/{}.text", name),
    )
}

fn us_compare(name: &str) {
    compare_fn(
        format!("tests/fixtures/markdown-rs/{}.html", name),
        format!("tests/fixtures/markdown-rs/{}.md", name),
    )
}

fn roundtrip(name: &str) {
    let html = format!("tests/fixtures/docs-maruku-unittest/{}.html", name);
    let text = format!("tests/fixtures/docs-maruku-unittest/{}.text", name);
    let mut comp = String::new();
    File::open(Path::new(&html))
        .unwrap()
        .read_to_string(&mut comp)
        .unwrap();
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
pub fn rt_alt() {
    roundtrip("alt")
}

#[test]
pub fn rt_blank() {
    roundtrip("blank")
}

#[test]
pub fn rt_blanks_in_code() {
    roundtrip("blanks_in_code")
}

#[test]
pub fn rt_code() {
    roundtrip("code")
}

#[test]
pub fn rt_code2() {
    roundtrip("code2")
}

#[test]
pub fn rt_code3() {
    roundtrip("code3")
}

#[test]
pub fn rt_easy() {
    roundtrip("easy")
}

#[test]
pub fn rt_headers() {
    roundtrip("headers")
}

//#[test]
//pub fn rt_entities() {
//roundtrip("entities")
//}

#[test]
pub fn rt_list1() {
    roundtrip("list1")
}

#[test]
pub fn rt_list2() {
    roundtrip("list2")
}

#[test]
pub fn rt_list3() {
    roundtrip("list3")
}

#[test]
pub fn rt_lists() {
    roundtrip("lists")
}

#[test]
pub fn rt_lists8() {
    roundtrip("lists8")
}

#[test]
pub fn rt_one() {
    roundtrip("one")
}

#[test]
pub fn rt_paragraph() {
    roundtrip("paragraph")
}

#[test]
pub fn rt_paragraphs() {
    roundtrip("paragraphs")
}

#[test]
pub fn rt_test() {
    roundtrip("test")
}

#[test]
pub fn rt_wrapping() {
    roundtrip("wrapping")
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
pub fn blanks_in_code() {
    compare("blanks_in_code")
}

#[test]
pub fn code() {
    compare("code")
}

#[test]
pub fn code2() {
    compare("code2")
}

#[test]
pub fn code3() {
    compare("code3")
}

#[test]
pub fn easy() {
    compare("easy")
}

#[test]
pub fn headers() {
    compare("headers")
}

//#[test]
//pub fn entities() {
//compare("entities")
//}

#[test]
pub fn list1() {
    compare("list1")
}

#[test]
pub fn list2() {
    compare("list2")
}

#[test]
pub fn list3() {
    compare("list3")
}

#[test]
pub fn lists() {
    compare("lists")
}

#[test]
pub fn lists8() {
    compare("lists8")
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

#[test]
pub fn wrapping() {
    compare("wrapping")
}

#[test]
pub fn ref_link() {
    us_compare("ref_link")
}

#[test]
pub fn ref_link_no_ref() {
    us_compare("ref_link_no_ref")
}

#[test]
pub fn ref_links() {
    us_compare("ref_links")
}

#[test]
pub fn ref_lnks_staggered() {
    us_compare("ref_links_staggered")
}
