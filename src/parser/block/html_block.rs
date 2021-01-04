use parser::Block;
use parser::Block::HtmlBlock;

const BLOCK_TAGS: [&str; 27] = [
    "p",
    "div",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "blockquote",
    "pre",
    "table",
    "dl",
    "ol",
    "ul",
    "script",
    "noscript",
    "form",
    "fieldset",
    "iframe",
    "math",
    "ins",
    "del",
    "hr",
    "hr/",
    "br",
    "br/",
    "!--", //Pseudo tag to deal with HTML comments
];

const SPECIAL_TAGS: [&str; 4] = ["hr", "hr/", "br", "br/"];

const COMMENT_START_TAG: &str = "<!--";
const COMMENT_END_TAG: &str = "-->";

fn find_tag(line: &str) -> Option<(&str, usize, usize)> {
    if let Some(head) = line.find(COMMENT_START_TAG) {
        let tail = head + 4;
        return Some(("!--", head, tail));
    } else if let Some(head) = line.find(COMMENT_END_TAG) {
        let tail = head + 3;
        return Some(("/!--", head, tail));
    } else if let Some(head) = line.find('<') {
        if let Some(len) = line[head..].find('>') {
            let tail = head + len + 1;
            let start = head + 1;
            let end;
            let mut fields = line[head..tail].split_whitespace();
            if let Some(tagid) = fields.next() {
                end = match fields.next() {
                    Some(_) => head + tagid.len(),
                    None => tail - 1,
                }
            } else {
                end = tail - 1;
            }
            if end < line.len() {
                let tag = &line[start..end];
                return Some((tag, head, tail));
            }
        }
    }
    None
}

pub fn parse_html_block(lines: &[&str]) -> Option<(Block, usize)> {
    let mut content = String::new();
    let mut line_count = 0;
    let mut nest_count = 0;
    let mut open_tag: &str = "";

    for line in lines {
        let line = line.trim_end();
        let mut offset = 0;
        line_count += 1;

        // Parse all HTML tags in the line.
        while let Some((tag, h, t)) = find_tag(&line[offset..]) {
            let idx = offset + h;
            offset += t;
            if !open_tag.is_empty() {
                if tag == open_tag {
                    // Deal with matching nested start-tags.
                    nest_count += 1;
                } else if let Some(tag) = tag.strip_prefix('/') {
                    // An end-tag was found.
                    if tag == open_tag {
                        if nest_count > 0 {
                            nest_count -= 1;
                        } else if offset == line.len() {
                            // The final matching end-tag was found.
                            content.push_str(line);
                            content.push('\n');
                            return Some((HtmlBlock(content), line_count));
                        } else {
                            return None;
                        }
                    }
                }
            } else if line_count == 1 && idx == 0 && BLOCK_TAGS.contains(&tag) {
                // A tag for the start of an HTML block was found (a start-tag).
                if SPECIAL_TAGS.contains(&tag) {
                    // Special case for single one line tags (like BR and HR)
                    content.push_str(line);
                    content.push('\n');
                    return Some((HtmlBlock(content), line_count));
                } else {
                    open_tag = tag;
                }
            } else {
                return None;
            }

            if offset >= line.len() {
                // End of the line.
                break;
            }
        }

        if !open_tag.is_empty() {
            content.push_str(line);
            content.push('\n');
        }
    }

    None
}

/***************************************************************************************************/
#[cfg(test)]
mod test {
    use super::find_tag;
    use super::parse_html_block;
    use parser::Block::HtmlBlock;

    #[test]
    fn find_tag_basic() {
        assert_eq!(find_tag("<div>"), Some(("div", 0, 5)));
        assert_eq!(find_tag("</div>"), Some(("/div", 0, 6)));
        assert_eq!(find_tag("  <div>  "), Some(("div", 2, 7)));
        assert_eq!(find_tag("  </div>  "), Some(("/div", 2, 8)));
        assert_eq!(find_tag("deadbeef<div>deadbeef"), Some(("div", 8, 13)));
    }

    #[test]
    fn find_tag_special() {
        assert_eq!(find_tag("<hr>"), Some(("hr", 0, 4)));
        assert_eq!(find_tag("<hr/>"), Some(("hr/", 0, 5)));
        assert_eq!(find_tag("<hr />"), Some(("hr", 0, 6)));
        assert_eq!(find_tag("  <hr>  "), Some(("hr", 2, 6)));
        assert_eq!(find_tag("  <hr/>  "), Some(("hr/", 2, 7)));
        assert_eq!(find_tag("  <hr />  "), Some(("hr", 2, 8)));
        assert_eq!(find_tag("  <hr class=\"thick\">  "), Some(("hr", 2, 20)));
        assert_eq!(find_tag(" <hr  class=\"thick\" />  "), Some(("hr", 1, 22)));
    }

    #[test]
    fn finds_block() {
        assert_eq!(
            parse_html_block(&vec!["<div>   ", "  the block  ", "</div>   "]).unwrap(),
            ((HtmlBlock("<div>\n  the block\n</div>\n".to_owned()), 3))
        );

        assert_eq!(
            parse_html_block(&vec!["<div>", "    more text", "</div>"]).unwrap(),
            ((HtmlBlock("<div>\n    more text\n</div>\n".to_owned()), 3))
        );

        assert_eq!(
            parse_html_block(&vec![
                "<div class=\"csv\">",
                "<table>",
                "</table>",
                "</div>"
            ])
            .unwrap(),
            ((
                HtmlBlock("<div class=\"csv\">\n<table>\n</table>\n</div>\n".to_owned()),
                4
            ))
        );
    }

    #[test]
    fn knows_when_to_stop() {
        assert_eq!(
            parse_html_block(&vec![
                "<div>",
                "    Test",
                "    this",
                "stuff",
                "    now",
                "</div>    "
            ])
            .unwrap(),
            ((
                HtmlBlock("<div>\n    Test\n    this\nstuff\n    now\n</div>\n".to_owned()),
                6
            ))
        );
    }

    #[test]
    fn no_false_positives() {
        assert_eq!(
            parse_html_block(&vec!["</table>", "    this", "stuff", "    now", "</div>"]),
            None
        );
        assert_eq!(
            parse_html_block(&vec!["<table>", "    this", "stuff", "    now", "</div>"]),
            None
        );
        assert_eq!(
            parse_html_block(&vec![
                "<table>",
                "<table>    this",
                "stuff",
                "    now",
                "</table>"
            ]),
            None
        );
    }

    #[test]
    fn no_early_matching() {
        assert_eq!(
            parse_html_block(&vec![" <div>", "    this", "stuff", "</div>"]),
            None
        );
        assert_eq!(
            parse_html_block(&vec!["xyz", "<div>", "    this", "stuff", "</div>"]),
            None
        );
        assert_eq!(
            parse_html_block(&vec!["xyz<div>", "    this", "stuff", "</div>"]),
            None
        );
        assert_eq!(
            parse_html_block(&vec!["<div>", "    this", "stuff", "</div>xyz"]),
            None
        );
        assert_eq!(
            parse_html_block(&vec!["wyz <div>", "    this", "stuff", "</div>"]),
            None
        );
        assert_eq!(
            parse_html_block(&vec!["<div>", "    this", "stuff", "</div> xyz "]),
            None
        );
    }

    #[test]
    fn ignore_nested_tags() {
        assert_eq!(
            parse_html_block(&vec![
                "<table>",
                "<table>",
                "<table></table>",
                "<table>",
                "</table>",
                "</table>",
                "</table>",
                "continued texted"
            ])
            .unwrap(),
            ((
                HtmlBlock(
                    "<table>\n<table>\n<table></table>\n<table>\n</table>\n</table>\n</table>\n"
                        .to_owned()
                ),
                7
            ))
        );

        assert_eq!(
            parse_html_block(&vec!["<div>","    <div>", "    this", "<div></div>", " is ", "<div>", "stuff" , "</div>", "    done", "</div> xyz </div>   ", "continued texted"]).unwrap(),
            ((HtmlBlock("<div>\n    <div>\n    this\n<div></div>\n is\n<div>\nstuff\n</div>\n    done\n</div> xyz </div>\n".to_owned()), 10))
        );
    }

    #[test]
    fn finds_html_comment_line() {
        assert_eq!(
            parse_html_block(&vec!["<!-- one line comment -->", "next line"]).unwrap(),
            ((HtmlBlock("<!-- one line comment -->\n".to_owned()), 1))
        );
        assert_eq!(
            parse_html_block(&vec!["<!--one line comment-->    "]).unwrap(),
            ((HtmlBlock("<!--one line comment-->\n".to_owned()), 1))
        );
    }

    #[test]
    fn finds_html_comment_block() {
        assert_eq!(
            parse_html_block(&vec![
                "<!--",
                "  this is a",
                "  comment block",
                "-->",
                "next line"
            ])
            .unwrap(),
            ((
                HtmlBlock("<!--\n  this is a\n  comment block\n-->\n".to_owned()),
                4
            ))
        );
        assert_eq!(
            parse_html_block(&vec![
                "<!-- followed by  ",
                "  another",
                "  comment block",
                "style-->",
                "next line"
            ])
            .unwrap(),
            ((
                HtmlBlock("<!-- followed by\n  another\n  comment block\nstyle-->\n".to_owned()),
                4
            ))
        );
    }

    #[test]
    fn finds_html_horizontal_rule() {
        assert_eq!(
            parse_html_block(&vec!["<hr>", "next line"]).unwrap(),
            ((HtmlBlock("<hr>\n".to_owned()), 1))
        );
        assert_eq!(
            parse_html_block(&vec!["<hr/>", "next line"]).unwrap(),
            ((HtmlBlock("<hr/>\n".to_owned()), 1))
        );
        assert_eq!(
            parse_html_block(&vec!["<hr \t/>", "next line"]).unwrap(),
            ((HtmlBlock("<hr \t/>\n".to_owned()), 1))
        );
    }

    #[test]
    fn finds_html_line_break() {
        assert_eq!(
            parse_html_block(&vec!["<br>", "next line"]).unwrap(),
            ((HtmlBlock("<br>\n".to_owned()), 1))
        );
        assert_eq!(
            parse_html_block(&vec!["<br/>", "next line"]).unwrap(),
            ((HtmlBlock("<br/>\n".to_owned()), 1))
        );
        assert_eq!(
            parse_html_block(&vec!["<br\t />", "next line"]).unwrap(),
            ((HtmlBlock("<br\t />\n".to_owned()), 1))
        );
    }
}
