use super::{Block, ListItem, Span};

trait JoinHelper<I> where I : Iterator {
    fn j(self, sep:&'static str) -> String;
}

impl<I> JoinHelper<I> for I where I : Iterator<Item=String> {
    fn j(self, sep:&'static str) -> String {
        self.collect::<Vec<String>>().join(sep)
    }
}

fn gen_block(b : Block) -> String {
    use Block::*;
    match b {
        Header(s, level) => format!("{} {}",
                ::std::iter::repeat("#".to_string()).take(level).j(""),
                generate_from_spans(s)
                ),
        Paragraph(s) => generate_from_spans(s),
        Blockquote(bb) => generate(bb).lines().map(|x|format!("> {}", x)).j("\n"),
        CodeBlock(x) => x.lines().map(|x|format!("    {}",x)).j("\n"),
        //OrderedList(Vec<ListItem>),
        UnorderedList(x) => generate_from_li(x),
        Raw(x) => x,
        Hr => "\n\n".to_string(),
    }
}

fn gen_span(s : Span) -> String {
    use Span::*;
    match s {
        Break => "  \n".to_string(),
        Text(x) => x,
        Code(x) => format!("`{}`",x),
        Link(a, b, None)     => format!("[{}]({})", a, b),
        Link(a, b, Some(c))  => format!("[{}]({} \"{}\")", a, b, c),
        Image(a, b, None)    => format!("![{}]({})", a, b),
        Image(a, b, Some(c)) => format!("![{}]({} \"{}\")", a, b, c),
    
        Emphasis(x) => format!("*{}*",   generate_from_spans(x)),
        Strong(x)   => format!("**{}**", generate_from_spans(x)),
    }
}


fn generate_from_li(data: Vec<ListItem>) -> String {
    use ListItem::*;

    data.into_iter().map(|x|format!("* {}", match x {
        Simple(x) => generate_from_spans(x),
        Paragraph(x) => format!("{}\n", 
                            generate(x)
                            .lines()
                            .enumerate()
                            .map(|(i, x)|
                                if i == 0 {
                                    x.to_string()
                                } else {
                                    format!("    {}", x)
                                }
                            ).j("\n")
                        ),
    })).j("\n")
}

fn generate_from_spans(data: Vec<Span>) -> String {
    data.into_iter().map(gen_span).j("")
}

pub fn generate(data: Vec<Block>) -> String {
    data.into_iter().map(gen_block).j("\n\n")
}
