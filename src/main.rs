mod parser;

fn main() {
    let text = " #### Hello World
                    ## Also
                    Stuff
                    ";
    parser::parse(text);
    let mut it = text.iter();
    println!("{}", it.next());
}

