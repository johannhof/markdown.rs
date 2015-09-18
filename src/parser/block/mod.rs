use parser::Block;
use parser::Block::{Paragraph, Raw};
use parser::Span::Text;
use parser::span::parse_spans;

mod atx_header;
mod setext_header;
mod hr;
mod code_block;
mod blockquote;
mod list;
use self::atx_header::parse_atx_header;
use self::setext_header::parse_setext_header;
use self::hr::parse_hr;
use self::code_block::parse_code_block;
use self::blockquote::parse_blockquote;
use self::list::parse_list;

pub fn parse_blocks (md : &str) -> Vec<Block> {
    let mut blocks = vec![];
    let mut t = vec![];
    let lines : Vec<&str> = md.lines_any().collect();
    let mut i = 0;
    while i < lines.len() {
        match parse_block(&lines[i .. lines.len()]){
            // if a block is found
            Some((block, consumed_lines)) => {
                // the current paragraph has ended,
                // push it to our blocks
                if !t.is_empty(){
                    blocks.push(Paragraph(t));
                    t = Vec::new();
                }
                blocks.push(block);
                i += consumed_lines;
            }
            // no known element, let's make this a paragraph
            None => {

                // empty linebreak => new paragraph
                if lines[i].is_empty() && !t.is_empty(){
                    blocks.push(Paragraph(t));
                    t = Vec::new();
                }

                // if the last item of the line is a text,
                if let Some(&Text(_)) = t.last() {
                    // add a whitespace between linebreaks
                    t.push(Text(" ".to_string()));
                }

                // TODO use append once it's stable
                for span in parse_spans(lines[i]) {
                    t.push(span);
                }
                i += 1;
            }
        }
    }
    if !t.is_empty(){
        blocks.push(Paragraph(t));
    }
    blocks
}

fn parse_block (lines: &[&str]) -> Option<(Block, usize)>{
    pipe_opt!(
        lines
        => parse_hr
        => parse_atx_header
        => parse_setext_header
        => parse_code_block
        => parse_blockquote
        => parse_list
    )
}

#[cfg(test)]
mod test {
    use super::parse_blocks;
    use parser::Block::{Header, Hr, CodeBlock, Paragraph, Blockquote};
    use parser::Span::Text;

    #[test]
    fn finds_atx_header() {
        assert_eq!(
            parse_blocks("### Test"),
            vec![Header(vec![Text("Test".to_string())], 3)]
        );
    }

    #[test]
    fn finds_setext_header() {
        assert_eq!(
            parse_blocks("Test\n-------"),
            vec![Header(vec![Text("Test".to_string())], 2)]
        );
        assert_eq!(
            parse_blocks("Test\n======="),
            vec![Header(vec![Text("Test".to_string())], 1)]
        );
    }

    #[test]
    fn finds_hr() {
        assert_eq!(
            parse_blocks("-------"),
            vec![Hr]
        );
        assert_eq!(
            parse_blocks("======="),
            vec![Hr]
        );
    }

    #[test]
    fn finds_code_block() {
        assert_eq!(
            parse_blocks("    this is code\n    and this as well"),
            vec![CodeBlock("this is code\nand this as well".to_string())]
        );
    }

    #[test]
    fn finds_blockquotes() {
        assert_eq!(
            parse_blocks("> One Paragraph\n>\n> ## H2 \n>\n"),
            vec![Blockquote(vec![Paragraph(vec![Text("One Paragraph".to_string())]), Header(vec![Text("H2".to_string())], 2)])]
        );

        assert_eq!(
            parse_blocks("> One Paragraph\n>\n> > Another blockquote\n>\n"),
            vec![Blockquote(vec![Paragraph(vec![Text("One Paragraph".to_string())]),
                       Blockquote(vec![Paragraph(vec![Text("Another blockquote".to_string())])])])]
        );

        assert_eq!(
            parse_blocks("> > One Paragraph\n> >\n> > Another blockquote\n>\n"),
            vec![Blockquote(vec![Blockquote(vec![Paragraph(vec![Text("One Paragraph".to_string())]),
                       Paragraph(vec![Text("Another blockquote".to_string())])])])]
        );

        assert_eq!(
            parse_blocks("> One Paragraph, just > text \n>\n"),
            vec![Blockquote(vec![Paragraph(vec![Text("One Paragraph, just > text ".to_string())])])]
        );

        assert_eq!(
            parse_blocks("> One Paragraph\n>\n> just > text \n>\n"),
            vec![Blockquote(vec![Paragraph(vec![Text("One Paragraph".to_string())]),Paragraph(vec![Text("just > text ".to_string())])])]
        );
    }
}


