use std::fs;

pub struct Markdown {
    pub contents: String,
}

#[derive(Debug)]
enum MarkdownElement {
    Header { text: String, level: u8 },
    CodeBlock { code: String, language: String },
}

impl Markdown {
    pub fn new(path: String) -> Markdown {
        if !path.ends_with(".md") {
            panic!("File must be a markdown file");
        }

        let contents = fs::read_to_string(path).expect("Failed to read file");
        Markdown { contents }
    }

    fn to_elements(&self) -> Vec<MarkdownElement> {
        let mut lines = self.contents.lines();
        let mut elements: Vec<MarkdownElement> = Vec::new();

        while let Some(line) = lines.next() {
            let trimmed_line = line.trim();
            if trimmed_line.starts_with('#') {
                // Count `#`
                let level = trimmed_line.chars().take_while(|&c| c == '#').count();
                // Check for space after #
                if level <= 6 && trimmed_line.chars().nth(level) == Some(' ') {
                    let text = trimmed_line.chars().skip(level + 1).collect::<String>();
                    elements.push(MarkdownElement::Header {
                        text,
                        level: level as u8,
                    });
                }
            } else if trimmed_line.starts_with("```") {
                let language = trimmed_line.chars().skip(3).collect::<String>();
                let mut code = String::new();
                while let Some(line) = lines.next() {
                    // Closing code block
                    if line.trim() == "```" {
                        break;
                    }
                    code.push_str(line);
                    code.push_str("\n");
                }
                elements.push(MarkdownElement::CodeBlock { code, language });
            }
        }

        elements
    }

    pub fn to_html(&self) -> String {
        let elements = self.to_elements();

        let mut html = String::new();

        for element in elements {
            match element {
                MarkdownElement::Header { text, level } => {
                    html.push_str(&format!("<h{}>{}</h{}>", level, text, level));
                }
                MarkdownElement::CodeBlock { code, language } => {
                    html.push_str(&format!(
                        "<pre><code class=\"language-{}\">{}</code></pre>",
                        language, code
                    ));
                }
            }
        }

        html
    }

    pub fn write_to_file(&self, path: String) {
        let html = self.to_html();
        fs::write(path, html).expect("Failed to write file");
    }
}
