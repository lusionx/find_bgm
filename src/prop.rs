use kuchiki::traits::*;
use kuchiki::NodeRef;

pub fn select_attributes(node: &NodeRef, attr: &str) -> Option<String> {
    if let Some(as_element) = node.as_element() {
        if let Ok(elem_atts) = as_element.attributes.try_borrow() {
            if let Some(val) = elem_atts.get(attr) {
                return Some(format!("{}", val));
            }
        }
    }
    None
}

pub fn serialize_text(node: &NodeRef) -> String {
    let mut result = String::new();
    for text_node in node.inclusive_descendants().text_nodes() {
        result.push_str(&text_node.borrow());
    }
    result.trim().to_string()
}

pub fn scan_node<F>(root: &NodeRef, query: &str, mut iter: F)
where
    F: FnMut(&NodeRef)
{
    for css_match in root
        .select(query)
        .expect(format!("Failed to parse CSS {}", query).as_str())
    {
        let node = css_match.as_node();
        iter(node)
    }
}
