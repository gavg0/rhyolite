use crate::walk_children;
use markup5ever_rcdom::Handle;
pub fn handle_header(tag: &str, node: &Handle, output: &mut String, depth: usize) {
    // Get header level from h1, h2, etc.
    if let Some(level) = tag.chars().nth(1) {

        let heading_level = level.to_digit(10).unwrap_or(0);

        if heading_level != 0 {
            output.push_str("\n");
        }
        // Add correct number of # symbols
        for _ in 0..heading_level {
            output.push('#');
        }
        output.push(' ');
        walk_children(node, output, depth);
        //output.push_str("\n");
    }
}