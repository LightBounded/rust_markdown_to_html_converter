mod md;

fn main() {
    let markdown = md::Markdown::new("assets/sample.md".to_string());
    let html = markdown.to_html();
    println!("{}", html);
}
