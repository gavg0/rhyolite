use crate::ElementHandler;
// use html5ever::Attribute;
use markup5ever_rcdom::Handle;
use crate::MarkdownConverter;

pub struct MarkHandler;
impl ElementHandler for MarkHandler {
    fn handle(&self, converter: &MarkdownConverter, node: &Handle, _attrs: &[html5ever::Attribute], output: &mut String, depth: usize) {
        output.push_str("==");
        converter.walk_children(node, output, depth);
        output.push_str("==");
    }
}

pub struct UnderlineHandler;
impl ElementHandler for UnderlineHandler {
    fn handle(&self, converter: &MarkdownConverter, node: &Handle, _attrs: &[html5ever::Attribute], output: &mut String, depth: usize) {
        output.push_str("<u>");
        converter.walk_children(node, output, depth);
        output.push_str("</u>");
    }
}

pub struct EmphasisHandler;
impl ElementHandler for EmphasisHandler {
    fn handle(&self, converter: &MarkdownConverter, node: &Handle, _attrs: &[html5ever::Attribute], output: &mut String, depth: usize) {
        output.push('*');
        converter.walk_children(node, output, depth);
        output.push('*');
    }
}

pub struct BoldHandler;
impl ElementHandler for BoldHandler {
    fn handle(&self, converter: &MarkdownConverter, node: &Handle, _attrs: &[html5ever::Attribute], output: &mut String, depth: usize) {
        output.push_str("**");
        converter.walk_children(node, output, depth);
        output.push_str("**");
    }
}

pub struct InlineCodeHandler;
impl ElementHandler for InlineCodeHandler {
    fn handle(&self, converter: &MarkdownConverter, node: &Handle, _attrs: &[html5ever::Attribute], output: &mut String, depth: usize) {
        output.push('`');
        converter.walk_children(node, output, depth);
        output.push('`');
    }
}

pub struct StrikeThroughHandler;
impl ElementHandler for StrikeThroughHandler {
    fn handle(&self, converter: &MarkdownConverter, node: &Handle, _attrs: &[html5ever::Attribute], output: &mut String, depth: usize) {
        output.push_str("~~");
        converter.walk_children(node, output, depth);
        output.push_str("~~");
    }
}

pub struct HorizontalRuleHandler;
impl ElementHandler for HorizontalRuleHandler {
    fn handle(&self, converter: &MarkdownConverter, node: &Handle, _attrs: &[html5ever::Attribute], output: &mut String, depth: usize) {
        output.push('\n');
        output.push_str("---");
        converter.walk_children(node, output, depth);
        // output.push_str("---");
        output.push('\n');
    }
}