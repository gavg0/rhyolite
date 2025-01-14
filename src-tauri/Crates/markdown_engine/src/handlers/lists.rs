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

pub struct OrderedListHandler;
impl ElementHandler for OrderedListHandler {
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
        let (is_ordered, list_index) = if let Some(parent) = node.parent.take() {
            let parent = parent.upgrade().unwrap();
            // Count previous list item siblings
            let list_index = parent.children.borrow()
                .iter()
                .take_while(|n| !std::ptr::eq(*n, node))
                .filter(|n| {
                    if let NodeData::Element { name, .. } = &n.data {
                        name.local.as_ref() == "li"
                    } else {
                        false
                    }
                })
                .count();
            
            if let NodeData::Element { name, .. } = &parent.data {
                (name.local.as_ref() == "ol", list_index + 1)
            } else {
                (false, 0)
            }
        } else {
            (false, 0)
        };

        // Add indentation based on depth
        output.push_str(&"  ".repeat(depth.saturating_sub(4)));
        
        // Add the appropriate list marker
        if is_ordered {
            output.push_str(&format!("{}. ", list_index));
        } else {
            output.push_str("- ");
        }
        
        converter.walk_children(node, output, depth);
        output.push('\n');
    }
}