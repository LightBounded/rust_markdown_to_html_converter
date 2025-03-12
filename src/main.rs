use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;
use std::env;
use std::fs;

#[derive(Parser)]
#[grammar = "lib/markdown.pest"] // Your pest grammar file for markdown
pub struct MarkdownParser;

#[derive(Debug)]
enum AstNode {
    Document(Vec<AstNode>),
    Paragraph(Vec<AstNode>),
    Emphasis(Vec<AstNode>),
    Strong(Vec<AstNode>),
    Text(String),
}

impl AstNode {
    fn to_html(&self) -> String {
        match self {
            AstNode::Document(children) => {
                // Combine all child HTML strings
                children.iter().map(|child| child.to_html()).collect()
            }
            AstNode::Paragraph(children) => {
                // Wrap paragraph content in <p> tags
                format!(
                    "<p>{}</p>",
                    children
                        .iter()
                        .map(|child| child.to_html())
                        .collect::<String>()
                )
            }
            AstNode::Emphasis(children) => {
                // Wrap content in <em> tags
                format!(
                    "<em>{}</em>",
                    children
                        .iter()
                        .map(|child| child.to_html())
                        .collect::<String>()
                )
            }
            AstNode::Strong(children) => {
                // Wrap content in <strong> tags
                format!(
                    "<strong>{}</strong>",
                    children
                        .iter()
                        .map(|child| child.to_html())
                        .collect::<String>()
                )
            }
            AstNode::Text(text) => text.clone(), // Return plain text
        }
    }
}

/// Recursively build an AST from a pest parse tree.
fn build_ast(pair: Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::document => {
            let children = pair
                .into_inner()
                .filter(|p| p.as_rule() != Rule::EOI)
                .map(build_ast)
                .collect();
            AstNode::Document(children)
        }
        Rule::paragraph => {
            let children = pair
                .into_inner()
                .filter(|p| p.as_rule() != Rule::EOI)
                .map(build_ast)
                .collect();
            AstNode::Paragraph(children)
        }
        Rule::emphasis => {
            let children = pair
                .into_inner()
                .filter(|p| p.as_rule() != Rule::EOI)
                .map(build_ast)
                .collect();
            AstNode::Emphasis(children)
        }
        Rule::strong => {
            let children = pair
                .into_inner()
                .filter(|p| p.as_rule() != Rule::EOI)
                .map(build_ast)
                .collect();
            AstNode::Strong(children)
        }
        Rule::text => AstNode::Text(pair.as_str().to_string()),
        Rule::EOI => {
            // Simply return an empty text node, or you can choose to ignore it.
            AstNode::Text(String::new())
        }
        _ => unreachable!("Unexpected rule: {:?}", pair.as_rule()),
    }
}

/// Pretty-print the AST with indentation.
fn print_ast(ast: &AstNode, indent: usize) {
    let padding = " ".repeat(indent);
    match ast {
        AstNode::Document(children) => {
            println!("{}Document", padding);
            for child in children {
                print_ast(child, indent + 2);
            }
        }
        AstNode::Paragraph(children) => {
            println!("{}Paragraph", padding);
            for child in children {
                print_ast(child, indent + 2);
            }
        }
        AstNode::Emphasis(children) => {
            println!("{}Emphasis", padding);
            for child in children {
                print_ast(child, indent + 2);
            }
        }
        AstNode::Strong(children) => {
            println!("{}Strong", padding);
            for child in children {
                print_ast(child, indent + 2);
            }
        }
        AstNode::Text(text) => {
            println!("{}Text: {:?}", padding, text);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the markdown file from the command line argument
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <markdown_file>", args[0]);
        return Ok(());
    }
    let filename = &args[1];
    let markdown = fs::read_to_string(filename)?;

    // Parse the markdown file using pest. Assuming 'document' is the starting rule.
    let mut pairs = MarkdownParser::parse(Rule::document, &markdown)?;
    println!("{:?}", pairs);
    let ast = build_ast(pairs.next().unwrap());

    // Print the AST in a nice, indented format.
    print_ast(&ast, 0);

    // Write to HTML file
    let html = ast.to_html();
    let html_filename = format!("{}.html", filename);
    fs::write(html_filename, html)?;

    Ok(())
}
