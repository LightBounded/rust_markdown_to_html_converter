mod md;

use std::fs;

fn main() {
    let markdown_contents = fs::read_to_string("assets/sample.md").expect("Failed to read file");
    let markdown = md::Markdown::new(markdown_contents);
    let md_elements = markdown.parse();
    for element in md_elements {
        println!("{:?}", element);
    }
}
