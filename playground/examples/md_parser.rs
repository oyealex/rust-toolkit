use nom::{
    IResult,
    bytes::complete::{tag, take_while, take_until},
    character::complete::{char, line_ending, not_line_ending, space0, space1},
    combinator::{map, opt, recognize},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    branch::alt,
};

#[derive(Debug, PartialEq)]
pub enum MarkdownElement {
    Heading(String, u8),
    Paragraph(String),
    CodeBlock(String),
    HtmlTag(String),
    Blockquote(String),
}

fn parse_heading(input: &str) -> IResult<&str, MarkdownElement> {
    let (input, level) = recognize(many1(char('#')))(input)?;
    let level = level.len() as u8;
    let (input, _) = space1(input)?;
    let (input, content) = not_line_ending(input)?;
    let (input, _) = opt(line_ending)(input)?;
    Ok((input, MarkdownElement::Heading(content.trim().to_string(), level)))
}

fn parse_code_block(input: &str) -> IResult<&str, MarkdownElement> {
    let (input, content) = delimited(tag("```"), take_until("```"), tag("```"))(input)?;
    let (input, _) = opt(line_ending)(input)?;
    Ok((input, MarkdownElement::CodeBlock(content.trim().to_string())))
}

fn parse_html_tag(input: &str) -> IResult<&str, MarkdownElement> {
    let (input, content) = delimited(char('<'), take_until(">"), char('>'))(input)?;
    let (input, _) = opt(line_ending)(input)?;
    Ok((input, MarkdownElement::HtmlTag(content.trim().to_string())))
}

fn parse_blockquote(input: &str) -> IResult<&str, MarkdownElement> {
    let (input, content) = preceded(char('>'), not_line_ending)(input)?;
    let (input, _) = opt(line_ending)(input)?;
    Ok((input, MarkdownElement::Blockquote(content.trim().to_string())))
}

fn parse_paragraph(input: &str) -> IResult<&str, MarkdownElement> {
    let (input, content) = not_line_ending(input)?;
    let (input, _) = opt(line_ending)(input)?;
    Ok((input, MarkdownElement::Paragraph(content.trim().to_string())))
}

fn parse_html_heading(input: &str) -> IResult<&str, MarkdownElement> {
    let (input, (level, content)) = tuple((
        alt((tag("<h1>"), tag("<h2>"), tag("<h3>"), tag("<h4>"), tag("<h5>"), tag("<h6>"))),
        take_until("</"),
    ))(input)?;
    let level = match level {
        "<h1>" => 1,
        "<h2>" => 2,
        "<h3>" => 3,
        "<h4>" => 4,
        "<h5>" => 5,
        "<h6>" => 6,
        _ => unreachable!(),
    };
    let (input, _) = take_until(">")(input)?;
    let (input, _) = opt(line_ending)(input)?;
    Ok((input, MarkdownElement::Heading(content.trim().to_string(), level)))
}

fn parse_markdown(input: &str) -> IResult<&str, Vec<MarkdownElement>> {
    many0(alt((
        parse_code_block,
        parse_html_tag,
        parse_blockquote,
        parse_html_heading,
        parse_heading,
        parse_paragraph,
    )))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_heading() {
        let input = "# Heading 1\n";
        let expected = MarkdownElement::Heading("Heading 1".to_string(), 1);
        let (_, result) = parse_heading(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_code_block() {
        let input = "```\n# Not a heading\n```\n";
        let expected = MarkdownElement::CodeBlock("# Not a heading".to_string());
        let (_, result) = parse_code_block(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_html_tag() {
        let input = "<div># Not a heading</div>\n";
        let expected = MarkdownElement::HtmlTag("div".to_string());
        let (_, result) = parse_html_tag(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_blockquote() {
        let input = "> # Not a heading\n";
        let expected = MarkdownElement::Blockquote("# Not a heading".to_string());
        let (_, result) = parse_blockquote(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_html_heading() {
        let input = "<h1>HTML Heading 1</h1>\n";
        let expected = MarkdownElement::Heading("HTML Heading 1".to_string(), 1);
        let (_, result) = parse_html_heading(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_markdown() {
        let input = "# Heading 1\nThis is a paragraph.\n```\n# Not a heading\n```\n<h1>HTML Heading 1</h1>\n";
        let expected = vec![
            MarkdownElement::Heading("Heading 1".to_string(), 1),
            MarkdownElement::Paragraph("This is a paragraph.".to_string()),
            MarkdownElement::CodeBlock("# Not a heading".to_string()),
            MarkdownElement::Heading("HTML Heading 1".to_string(), 1),
        ];
        let (_, result) = parse_markdown(input).unwrap();
        assert_eq!(result, expected);
    }
}


fn main() {

}