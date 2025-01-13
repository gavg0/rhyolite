use crate::walk_children;
use markup5ever_rcdom::Handle;
pub fn handle_paragraph(tag: &str, node: &Handle, output: &mut String, depth: usize) {
    if !tag.starts_with('p') {
        return;
    }
    output.push_str("\n");
    walk_children(node, output, depth);
}