use std::path::Path;
use std::fs::File;
use std::io::Read;
use markdown;
use difference;

fn compare(name: &str) {
    let html = format!("tests/fixtures/{}.html", name);
    let text = format!("tests/fixtures/{}.text", name);
    let mut comp = String::new();
    File::open(Path::new(&html)).unwrap().read_to_string(&mut comp).unwrap();
    let md = Path::new(&text);

    let mut tokens = String::new();
    File::open(md).unwrap().read_to_string(&mut tokens).unwrap();
    println!("{:?}", markdown::tokenize(&tokens));

    difference::assert_diff(&comp, &markdown::file_to_html(md).unwrap(), " ", 0);
}

fn roundtrip(name: &str) {
    let html = format!("tests/fixtures/{}.html", name);
    let text = format!("tests/fixtures/{}.text", name);
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
pub fn rt_alt() {
    roundtrip("docs-maruku-unittest/alt")
}

#[test]
pub fn rt_blank() {
    roundtrip("docs-maruku-unittest/blank")
}


#[test]
pub fn rt_blanks_in_code() {
    roundtrip("docs-maruku-unittest/blanks_in_code")
}


#[test]
pub fn rt_code() {
    roundtrip("docs-maruku-unittest/code")
}


#[test]
pub fn rt_code2() {
    roundtrip("docs-maruku-unittest/code2")
}


#[test]
pub fn rt_code3() {
    roundtrip("docs-maruku-unittest/code3")
}

#[test]
pub fn rt_easy() {
    roundtrip("docs-maruku-unittest/easy")
}

#[test]
pub fn rt_headers() {
    roundtrip("docs-maruku-unittest/headers")
}

//#[test]
//pub fn rt_entities() {
    //roundtrip("docs-maruku-unittest/entities")
//}

#[test]
pub fn rt_list1() {
    roundtrip("docs-maruku-unittest/list1")
}

#[test]
pub fn rt_list2() {
    roundtrip("docs-maruku-unittest/list2")
}

#[test]
pub fn rt_list3() {
    roundtrip("docs-maruku-unittest/list3")
}

#[test]
pub fn rt_lists() {
    roundtrip("docs-maruku-unittest/lists")
}

#[test]
pub fn rt_lists8() {
    roundtrip("docs-maruku-unittest/lists8")
}

#[test]
pub fn rt_one() {
    roundtrip("docs-maruku-unittest/one")
}

#[test]
pub fn rt_paragraph() {
    roundtrip("docs-maruku-unittest/paragraph")
}

#[test]
pub fn rt_paragraphs() {
    roundtrip("docs-maruku-unittest/paragraphs")
}

#[test]
pub fn rt_test() {
    roundtrip("docs-maruku-unittest/test")
}

#[test]
pub fn rt_wrapping() {
    roundtrip("docs-maruku-unittest/wrapping")
}

#[test]
pub fn utf8() {
    compare("custom/utf8")
}

#[test]
pub fn rt_utf8() {
    roundtrip("custom/utf8")
}

#[test]
pub fn alt() {
    compare("docs-maruku-unittest/alt")
}

#[test]
pub fn blank() {
    compare("docs-maruku-unittest/blank")
}

#[test]
pub fn blanks_in_code() {
    compare("docs-maruku-unittest/blanks_in_code")
}

#[test]
pub fn code() {
    compare("docs-maruku-unittest/code")
}

#[test]
pub fn code2() {
    compare("docs-maruku-unittest/code2")
}

#[test]
pub fn code3() {
    compare("docs-maruku-unittest/code3")
}

#[test]
pub fn easy() {
    compare("docs-maruku-unittest/easy")
}

#[test]
pub fn headers() {
    compare("docs-maruku-unittest/headers")
}

//#[test]
//pub fn entities() {
    //compare("docs-maruku-unittest/entities")
//}

#[test]
pub fn list1() {
    compare("docs-maruku-unittest/list1")
}

#[test]
pub fn list2() {
    compare("docs-maruku-unittest/list2")
}

#[test]
pub fn list3() {
    compare("docs-maruku-unittest/list3")
}

#[test]
pub fn lists() {
    compare("docs-maruku-unittest/lists")
}

#[test]
pub fn lists8() {
    compare("docs-maruku-unittest/lists8")
}

#[test]
pub fn one() {
    compare("docs-maruku-unittest/one")
}

#[test]
pub fn paragraph() {
    compare("docs-maruku-unittest/paragraph")
}

#[test]
pub fn paragraphs() {
    compare("docs-maruku-unittest/paragraphs")
}

#[test]
pub fn test() {
    compare("docs-maruku-unittest/test")
}

#[test]
pub fn wrapping() {
    compare("docs-maruku-unittest/wrapping")
}

