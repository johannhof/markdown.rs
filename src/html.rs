use parser;

pub fn to_html (tokens : Vec<parser::Token>) -> String {
    let mut ret = String::new();
    for token in tokens.iter(){
        let next = match token {
            &parser::Token { kind: "Header", md: text} => format!("<h1>{}</h1>", text),
            _ => format!("wat")
        };
        ret.push_str(next.as_slice())
    }
    ret
}
