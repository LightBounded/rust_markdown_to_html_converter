use std::fs;

pub struct Markdown {
    pub contents: String,
}

#[derive(Debug)]
enum MarkdownElement {
    Header { text: String, level: u8 },
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
        let lines = self.contents.lines();
        let mut elements: Vec<MarkdownElement> = Vec::new();
        for line in lines {
            let trimmed_line = line.trim();
            if trimmed_line.starts_with('#') {
                let level = trimmed_line.chars().take_while(|&c| c == '#').count(); // Count `#`
                // 🔹 Check for space after #
                if level <= 6 && trimmed_line.chars().nth(level) == Some(' ') {
                    let text = trimmed_line.chars().skip(level + 1).collect::<String>();
                    elements.push(MarkdownElement::Header {
                        text,
                        level: level as u8,
                    });
                }
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
            }
        }

        html
    }
}
