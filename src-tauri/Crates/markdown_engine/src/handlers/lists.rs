use crate::ElementHandler;
// use html5ever::Attribute;
use markup5ever_rcdom::{NodeData, Handle};
use crate::MarkdownConverter;

pub struct UnorderedListHandler;
impl ElementHandler for UnorderedListHandler {
    fn handle(&self, converter: &MarkdownConverter, node: &Handle, _attrs: &[html5ever::Attribute], output: &mut String, depth: usize) {
        output.push('\n');
        converter.walk_children(node, output, depth);
        output.push('\n');
    }
}

pub struct ListItemHandler;
impl ElementHandler for ListItemHandler {
    fn handle(&self, converter: &MarkdownConverter, node: &Handle, _attrs: &[html5ever::Attribute], output: &mut String, depth: usize) {
        // Check if parent is ordered or unordered list
        let is_ordered = if let Some(parent) = node.parent.take() {
            if let NodeData::Element { name, .. } = &parent.upgrade().unwrap().data {
                name.local.as_ref() == "ol"
            } else {
                false
            }
        } else {
            false
        };

        // Add indentation based on depth
        // output.push_str(&"  ".repeat(depth.saturating_sub(1)));
        
        // Add the appropriate list marker
        if is_ordered {
            output.push_str("1. "); // Markdown will auto-number these correctly
        } else {
            output.push_str("- ");
        }
        
        converter.walk_children(node, output, depth);
        output.push('\n');
    }
}