// html_processor_mod.rs

use crate::html_templating_data_trait_mod::*;
use crate::trait_utils_mod;
use dev_bestia_string_utils::*;

use lazy_static::lazy_static;
use unwrap::unwrap;

// conditional only for wasm (on web-browser client)
#[cfg(target_arch = "wasm32")]
lazy_static! {
    pub static ref PREFIX: PrefixForTemplateVariables = PrefixForTemplateVariables {
        text: s!("wt_"),
        url: s!("wu_"),
        exist: s!("wb_"),
        node: s!("wn_"),
        subtemplate: s!("wtmplt_"),
        subtemplate_comment: s!("<!--wtmplt_"),
        attr_text: s!("data-wt_"),
        attr_url: s!("data-wu_"),
        attr_exist: s!("data-wb_"),
    };
}

// conditional only for non-wasm (on server)
#[cfg(not(target_arch = "wasm32"))]
lazy_static! {
    pub static ref PREFIX: PrefixForTemplateVariables = PrefixForTemplateVariables {
        text: s!("st_"),
        url: s!("su_"),
        exist: s!("sb_"),
        node: s!("sn_"),
        subtemplate: s!("stmplt_"),
        subtemplate_comment: s!("<!--stmplt_"),
        attr_text: s!("data-st_"),
        attr_url: s!("data-su_"),
        attr_exist: s!("data-sb_"),
    };
}

/// process root template (not subtemplates) from string
pub fn process_html<T: HtmlTemplatingDataTrait>(self_0: &T, html_template_raw: &str) -> String {
    // remove the only element that is not MicroXml compatible. It will be added later.
    let html_template_raw = html_template_raw
        .trim()
        .trim_start_matches("<!DOCTYPE html>");

    let nodes = unwrap!(trait_utils_mod::process_template_raw_to_nodes(
        self_0,
        &html_template_raw,
        trait_utils_mod::HtmlOrSvg::Html,
        "",
        0
    ));
    // because this is the root template it must return one ElementNode
    let mut html = s!();
    match &nodes[0] {
        trait_utils_mod::Node::Element(temp_element_node) => {
            html = unwrap!(root_element_node_to_html_string(temp_element_node));
        }
        _ => {
            log::error!("Error: process_template_raw_to_nodes() does not return one ElementNode.")
        }
    }
    // return
    html
}

/// converts element node to string
/// the attribute values and Text nodes and Comments are xml encoded
fn root_element_node_to_html_string(element_node: &ElementNode) -> Result<String, String> {
    let mut dom_path: Vec<String> = Vec::new();
    /// recursive private fn sub element to html
    fn sub_element_node_mut_html(
        html: &mut String,
        element_node: &ElementNode,
        dom_path: &mut Vec<String>,
    ) {
        html.push_str("\n");
        html.push_str("    ".repeat(dom_path.len()).as_str());
        html.push_str("<");
        html.push_str(&element_node.tag_name);
        dom_path.push(s!(element_node.tag_name));
        for attr in element_node.attributes.iter() {
            html.push_str(" ");
            html.push_str(&attr.name);
            html.push_str("=\"");
            html.push_str(&encode_5_xml_control_characters(&attr.value));
            html.push_str("\"");
        }
        if element_node.is_self_closing == true {
            // auto-closing element
            // for <br /> is significant to stay auto-closed
            // because <br></br> is rendered differently
            html.push_str(" />");
            unwrap!(dom_path.pop());
        // dbg!(&html);
        } else {
            html.push_str(">");
            for sub_elem in element_node.children.iter() {
                match &sub_elem {
                    trait_utils_mod::Node::Element(sub_element) => {
                        // recursion
                        sub_element_node_mut_html(html, sub_element, dom_path);
                    }
                    trait_utils_mod::Node::Text(text) => {
                        if unwrap!(dom_path.last()) == "script" {
                            // in html script elements are encoded differently
                            html.push_str(&encode_html_script_node(&text));
                        } else {
                            html.push_str(&encode_5_xml_control_characters(&text));
                        }
                    }
                    trait_utils_mod::Node::Comment(text) => html.push_str(&format!(
                        "<!--{}-->",
                        encode_5_xml_control_characters(&text)
                    )),
                }
            }
            // end tag
            if ["head", "div","body","html"].contains(&element_node.tag_name.as_str()){
                html.push_str("\n");    
                html.push_str("    ".repeat(dom_path.len()-1).as_str());
            }
            html.push_str("</");
            html.push_str(&element_node.tag_name);
            html.push_str(">");
            unwrap!(dom_path.pop());
        }
    }

    let mut html = String::with_capacity(5000);
    html.push_str("<!DOCTYPE html>");
    sub_element_node_mut_html(&mut html, element_node, &mut dom_path);
    // return
    Ok(html)
}
