use html2md::parse_html; //html2md module to convert html to markdown
use pulldown_cmark::{html, Options, Parser}; //pulldown_cmark module to parse markdown
use mdka; //mdka module to convert markdown to html
use regex::Regex; //regex module to use Regex type

pub fn html_to_markdown(html: &str) -> String {
    let re = Regex::new(r"<mark>(.*?)</mark>").unwrap();
    re.replace_all(html, "==$1==").to_string()
}