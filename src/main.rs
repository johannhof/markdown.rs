extern crate markdown;

fn main() {
    let path = Path::new("test.md");
    let display = path.display();

    println!("{}", markdown::file_to_html(&path));
}

