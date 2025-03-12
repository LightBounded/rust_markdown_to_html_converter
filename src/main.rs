mod md;

fn main() {
    let markdown = md::Markdown::new("assets/sample.md".to_string());
    markdown.write_to_file("assets/sample.html".to_string());
}
