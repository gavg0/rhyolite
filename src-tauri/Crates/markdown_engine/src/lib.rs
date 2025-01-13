use html5ever::driver::ParseOpts;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::{parse_document, serialize};
use html5ever::tree_builder::TreeSink; 
use markup5ever_rcdom::{RcDom, NodeData, Handle};

mod headers;
mod paragraph;
mod links;
mod basic_elements;

pub fn convert_to_markdown(html: &String) -> String {
    let dom = parse_to_dom(html.to_string());
    let mut markdown = String::new();
    println!("{}", html);
    print_dom(&dom.document, 0);
    traverse_dom(&dom.document, &mut markdown, 0);
    markdown
}

fn print_dom(node: &Handle, indent: usize) {
    let indent_str = " ".repeat(indent);

    match node.data {
        NodeData::Element { ref name, ref attrs, .. } => {
            // Print element name and attributes
            println!("{}Element: {}", indent_str, name.local);
            for attr in attrs.borrow().iter() {
                println!("{}  Attr: {} = {}", indent_str, attr.name.local, attr.value);
            }
            // Print children
            for child in node.children.borrow().iter() {
                print_dom(child, indent + 2);
            }
        },
        NodeData::Text { ref contents } => {
            println!("{}Text: {}", indent_str, contents.borrow());
        },
        NodeData::Document => {
            println!("{}Document", indent_str);
            for child in node.children.borrow().iter() {
                print_dom(child, indent + 2);
            }
        },
        _ => println!("{}Other node type", indent_str),
    }
}

fn parse_to_dom(html: String) -> RcDom {
    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let dom: RcDom = parse_document(RcDom::default(), opts)
        .from_utf8()
        .read_from(&mut html.as_bytes())
        .unwrap();
    dom
}

fn traverse_dom(node: &Handle, output: &mut String, depth: usize) {
    match node.data {
        // If we find text content
        NodeData::Text { ref contents } => {
            let text = contents.borrow();
            if !text.trim().is_empty() {  // Only add non-empty text
                output.push_str(&text);
            }
        },
        // If we find an HTML element (like p, h1, h2, a)
        NodeData::Element { ref name, ref attrs, .. } => {
            let element_tag = name.local.as_ref();

                match element_tag {
                    tag if tag.starts_with('h') => headers::handle_header(tag, node, output, depth),
                    // For paragraphs: add newlines before
                    "p" => {
                        output.push_str("\n");
                        walk_children(node, output, depth);
                        // output.push_str("\n");
                    },
                    // For links: convert to Markdown link format [text](url)
                    "a" => {
                        output.push('[');
                        walk_children(node, output, depth);  // Get link text
                        output.push(']');
                        output.push('(');
                        // Find and add the href URL
                        if let Some(attr) = attrs.borrow().iter().find(|attr|
                            attr.name.local.as_ref() == "href") {
                            output.push_str(&attr.value);
                        }
                        output.push(')');
                    },
                    // For any other elements, just process their children
                    _ => walk_children(node, output, depth),
                }
        },
        // For any other type of node, process its children
        _ => walk_children(node, output, depth),
    }
}

fn walk_children(node: &Handle, output: &mut String, depth: usize) {
    for child in node.children.borrow().iter() {
        traverse_dom(child, output, depth + 1);
    }
}