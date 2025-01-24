use crate::ElementHandler;
use crate::MarkdownConverter;
use html5ever::Attribute;
use markup5ever_rcdom::Handle;

pub struct HeaderHandler {
    level: usize,
}

impl HeaderHandler {
    pub fn level(level: usize) -> Self {
        Self { level }
    }
}

impl ElementHandler for HeaderHandler {
    fn handle(
        &self,
        converter: &MarkdownConverter,
        node: &Handle,
        _attrs: &[Attribute],
        output: &mut String,
        depth: usize,
    ) {
        output.push_str("\n");
        output.push_str(&"#".repeat(self.level));
        output.push(' ');
        converter.walk_children(node, output, depth);
        // output.push_str("\n");
    }
}
