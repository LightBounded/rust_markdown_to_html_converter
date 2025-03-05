#[derive(Debug)]

pub struct Markdown {
    content: String,
}

impl Markdown {
    pub fn new(content: String) -> Self {
        Markdown { content }
    }

    pub fn parse(&self) -> Vec<MarkdownElement> {
        let mut elements = Vec::new();
        let mut in_code_block = false;

        for line in self.content.lines() {
            if line.starts_with('#') {
                elements.push(parse_header(line));
                continue;
            }

            if line.starts_with("- ") {
                // Check if last pushed element is a list
                if let Some(MarkdownElement::List(current_list)) = elements.last_mut() {
                    current_list.push(line[2..].to_string());
                } else {
                    let mut new_list = Vec::new();
                    new_list.push(line[2..].to_string());
                    elements.push(MarkdownElement::List(new_list));
                }
            }

            if line.starts_with("```") {
                // Check if last pushed element is a code block
                if in_code_block {
                    in_code_block = false;
                } else {
                    // The language should be the first word after the ```
                    let language = line[3..].trim().to_string();
                    in_code_block = true;
                    elements.push(MarkdownElement::CodeBlock {
                        language,
                        text: String::new(),
                    });
                }
            }

            if in_code_block && !line.starts_with("```") {
                if let Some(MarkdownElement::CodeBlock { text, .. }) = elements.last_mut() {
                    *text += line;
                }
            }
        }

        elements
    }
}

#[derive(Debug)]
pub enum MarkdownElement {
    Heading { level: u8, text: String },
    Paragraph(Vec<InlineMarkdownElement>), // Paragraphs contain inline elements
    List(Vec<String>),                     // A list contains multiple items
    CodeBlock { language: String, text: String },
    BlockQuote(String),
    HorizontalRule,
}

#[derive(Debug)]
pub enum InlineMarkdownElement {
    Text(String),
    Bold(String),
    Italic(String),
    InlineCode(String),
    Link { text: String, url: String },
}

fn parse_header(line: &str) -> MarkdownElement {
    let mut text = String::new();
    let mut level = 0;
    let mut text_start = false;
    for c in line.chars() {
        if c == '#' && !text_start {
            level += 1;
        } else if c == ' ' && !text_start {
            text_start = true;
        } else {
            text.push(c);
        }
    }

    MarkdownElement::Heading { level, text }
}
