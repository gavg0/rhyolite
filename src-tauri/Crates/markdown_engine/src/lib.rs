use html5ever::driver::ParseOpts;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::{parse_document, serialize};
use html5ever::tree_builder::TreeSink;
use markup5ever_rcdom::{RcDom, NodeData, Handle};
use std::borrow::Cow;
use std::collections::HashMap;

// Define element handlers as trait for better extensibility
pub trait ElementHandler {
    fn handle(&self, converter: &MarkdownConverter, node: &Handle, attrs: &[html5ever::Attribute], output: &mut String, depth: usize);
}

// Struct to hold element handlers
pub struct MarkdownConverter {
    handlers: HashMap<&'static str, Box<dyn ElementHandler>>,
}

impl MarkdownConverter {
    pub fn new() -> Self {
        let mut converter = Self { 
            handlers: HashMap::new() 
        };
        
        // Register handlers for different elements
        converter.handlers.insert("p", Box::new(ParagraphHandler));
        converter.handlers.insert("mark", Box::new(MarkHandler));
        converter.handlers.insert("em", Box::new(EmphasisHandler));
        converter.handlers.insert("a", Box::new(LinkHandler));

        // Register handlers for headers
        converter.handlers.insert("h1", Box::new(HeaderHandler::level(1)));
        converter.handlers.insert("h2", Box::new(HeaderHandler::level(2)));
        converter.handlers.insert("h3", Box::new(HeaderHandler::level(3)));
        converter.handlers.insert("h4", Box::new(HeaderHandler::level(4)));
        converter.handlers.insert("h5", Box::new(HeaderHandler::level(5)));
        converter.handlers.insert("h6", Box::new(HeaderHandler::level(6)));
        
        converter
    }

    pub fn convert_to_markdown(&self, html: &str) -> String {
        let dom = self.parse_to_dom(html);
        let mut markdown = String::with_capacity(html.len());
        self.traverse_dom(&dom.document, &mut markdown, 0);
        markdown.trim().to_string()
    }

    fn parse_to_dom(&self, html: &str) -> RcDom {
        let opts = ParseOpts {
            tree_builder: TreeBuilderOpts {
                drop_doctype: true,
                ..Default::default()
            },
            ..Default::default()
        };

        parse_document(RcDom::default(), opts)
            .from_utf8()
            .read_from(&mut html.as_bytes())
            .unwrap()
    }

    fn traverse_dom(&self, node: &Handle, output: &mut String, depth: usize) {
        match &node.data {
            NodeData::Text { contents } => {
                let text = contents.borrow();
                if !text.trim().is_empty() {
                    output.push_str(&text);
                }
            },
            NodeData::Element { name, attrs, .. } => {
                let tag_name = name.local.as_ref();

                // Use registered handler if available
                if let Some(handler) = self.handlers.get(tag_name) {
                    handler.handle(self, node, &attrs.borrow(), output, depth);
                } else {
                    // Default behavior for unknown elements
                    self.walk_children(node, output, depth);
                }
            },
            _ => self.walk_children(node, output, depth),
        }
    }

    pub fn walk_children(&self, node: &Handle, output: &mut String, depth: usize) {
        for child in node.children.borrow().iter() {
            self.traverse_dom(child, output, depth + 1);
        }
    }
}

// Implementation of handlers
// Header handler implementation
struct HeaderHandler {
    level: usize,
}

impl HeaderHandler {
    fn level(level: usize) -> Self {
        Self { level }
    }
}

impl ElementHandler for HeaderHandler {
    fn handle(&self, converter: &MarkdownConverter, node: &Handle, _attrs: &[html5ever::Attribute], output: &mut String, depth: usize) {
        output.push_str("\n");
        output.push_str(&"#".repeat(self.level));
        output.push(' ');
        converter.walk_children(node, output, depth);
        // output.push_str("\n");
    }
}

struct ParagraphHandler;
impl ElementHandler for ParagraphHandler {
    fn handle(&self, converter: &MarkdownConverter, node: &Handle, _attrs: &[html5ever::Attribute], output: &mut String, depth: usize) {
        output.push_str("\n");
        converter.walk_children(node, output, depth);
        // output.push_str("\n");
    }
}

struct MarkHandler;
impl ElementHandler for MarkHandler {
    fn handle(&self, converter: &MarkdownConverter, node: &Handle, _attrs: &[html5ever::Attribute], output: &mut String, depth: usize) {
        output.push_str("==");
        converter.walk_children(node, output, depth);
        output.push_str("==");
    }
}

struct EmphasisHandler;
impl ElementHandler for EmphasisHandler {
    fn handle(&self, converter: &MarkdownConverter, node: &Handle, _attrs: &[html5ever::Attribute], output: &mut String, depth: usize) {
        output.push('*');
        converter.walk_children(node, output, depth);
        output.push('*');
    }
}

struct LinkHandler;
impl ElementHandler for LinkHandler {
    fn handle(&self, converter: &MarkdownConverter, node: &Handle, attrs: &[html5ever::Attribute], output: &mut String, depth: usize) {
        output.push('[');
        converter.walk_children(node, output, depth);
        output.push(']');
        output.push('(');
        
        if let Some(href) = attrs.iter().find(|attr| attr.name.local.as_ref() == "href") {
            output.push_str(&href.value);
        }
        
        output.push(')');
    }
}

// Public interface
pub fn convert_to_markdown(html: &str) -> String {
    MarkdownConverter::new().convert_to_markdown(html)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_paragraph() {
        let html = "<p>Hello World</p>";
        let result = convert_to_markdown(html);
        assert_eq!(result.trim(), "Hello World");
    }

    #[test]
    fn test_emphasis() {
        let html = "<em>emphasized text</em>";
        let result = convert_to_markdown(html);
        assert_eq!(result.trim(), "*emphasized text*");
    }

    #[test]
    fn test_link() {
        let html = r#"<a href="https://example.com">Link text</a>"#;
        let result = convert_to_markdown(html);
        assert_eq!(result.trim(), "[Link text](https://example.com)");
    }

    #[test]
    fn test_header() {
        let html = "<h1>Header</h1>";
        let result = convert_to_markdown(html);
        assert_eq!(result.trim(), "# Header");
    }
}