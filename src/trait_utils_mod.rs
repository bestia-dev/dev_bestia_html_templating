// trait_utils_mod.rs

use crate::html_processor_mod::PREFIX;
use crate::html_templating_data_trait_mod::*;

use dev_bestia_string_utils::*;
use dev_bestia_url_utf8::url_u;
use dev_bestia_url_utf8::UrlUtf8EncodedString;
use reader_for_microxml::*;

use unwrap::unwrap;

/// Svg elements are different because they have a namespace
#[derive(Clone, Copy)]
pub enum HtmlOrSvg {
    // html element
    Html,
    // svg element
    Svg,
}

#[derive(Clone, Debug)]
pub struct SubTemplate {
    pub name: String,
    pub placeholder: String,
    pub template: String,
}

#[derive(Clone, Debug)]
pub enum Node {
    /// A text node. The text must be not encoded.
    /// It will be xml encoded when converting the node to html string.
    Text(String),
    /// An element potentially with attributes and children.
    Element(ElementNode),
    /// comment. . The text must be not encoded.
    /// It will be xml encoded when converting the node to html string.
    Comment(String),
}

/// this is used for templates and subtemplates equally
/// first extracts all children sub_templates
/// returns Nodes
pub fn process_template_raw_to_nodes<T: HtmlTemplatingDataTrait>(
    self_0: &T,
    html_template_raw: &str,
    html_or_svg_parent: HtmlOrSvg,
    subtemplate_name: &str,
    row_number: usize,
) -> Result<Vec<Node>, &'static str> {
    // html_template_raw can be a fragment. I add the root, that will later be removed.
    let html_template_raw = &format!("<template>{}</template>", html_template_raw);
    // extract sub_templates. Only one level deep.
    let sub_templates = extract_children_sub_templates(html_template_raw);
    // the index zero is the drained main template
    let mut reader_for_microxml = ReaderForMicroXml::new(&sub_templates[0].template);
    let mut dom_path: Vec<String> = Vec::new();
    let mut root_element;
    let mut html_or_svg_local = html_or_svg_parent;

    // the root element must be only one
    if let Some(result_token) = reader_for_microxml.next() {
        match result_token {
            Ok(token) => {
                match token {
                    Token::StartElement(tag_name) => {
                        dom_path.push(s!(tag_name));
                        root_element = ElementNode {
                            tag_name: s!(tag_name),
                            ..Default::default()
                        };
                        if &tag_name == &"svg" {
                            html_or_svg_local = HtmlOrSvg::Svg;
                        }
                        if let HtmlOrSvg::Svg = html_or_svg_local {
                            // svg elements have this namespace
                            root_element.namespace = Some(s!("http://www.w3.org/2000/svg"));
                        }
                        // recursive function can return error

                        match unwrap!(fill_element_node(
                            self_0,
                            &mut reader_for_microxml,
                            root_element,
                            html_or_svg_local,
                            &mut dom_path,
                            &sub_templates,
                            subtemplate_name,
                            row_number,
                            // exists_next_node_or_attribute:
                            true,
                        )) {
                            Ok(new_root_element) => root_element = new_root_element,
                            Err(err) => {
                                return Err(&err);
                            }
                        }
                    }
                    _ => {
                        // return error
                        return Err("Error: no root element");
                    }
                }
            }
            Err(err_msg) => return Err(err_msg),
        }
    } else {
        return Err("Error: Not found root element.");
    }
    // remove the added root <template>
    // return its children
    Ok(root_element.children)
}

/// boilerplate
pub fn match_else_for_replace_with_nodes(data_model_name: &str, placeholder: &str) -> Vec<Node> {
    let err_msg = format!(
        "Error: Unrecognized {} replace_with_nodes: \"{}\"",
        data_model_name, placeholder
    );
    log::error!("{}", &err_msg);
    let node = Node::Element(ElementNode {
        tag_name: s!("h2"),
        children: vec![Node::Text(err_msg)],
        ..Default::default()
    });
    return vec![node];
}

/// boilerplate
pub fn match_else_for_exists_next_node_or_attribute(
    data_model_name: &str,
    placeholder: &str,
) -> bool {
    log::error!(
        "Error: Unrecognized {} exists_next_node_or_attribute: \"{}\"",
        data_model_name,
        placeholder
    );
    true
}
/// boilerplate
pub fn match_else_for_replace_with_string(data_model_name: &str, placeholder: &str) -> String {
    let err_msg = format!(
        "Error: Unrecognized {} replace_with_string: \"{}\"",
        data_model_name, placeholder
    );
    log::error!("{}", &err_msg);
    s!(err_msg)
}
/// boilerplate
pub fn match_else_for_replace_with_url(
    data_model_name: &str,
    placeholder: &str,
) -> UrlUtf8EncodedString {
    let err_msg = format!(
        "Error: Unrecognized {} replace_with_url: \"{}\"",
        data_model_name, placeholder
    );
    log::error!("{}", &err_msg);
    url_u!(&err_msg, "")
}

///boilerplate
pub fn match_else_for_process_sub_template(
    data_model_name: &str,
    template_name: &str,
) -> Vec<Node> {
    let err_msg = format!(
        "Error: Unrecognized {} process_sub_template: \"{}\"",
        data_model_name, template_name
    );
    log::error!("{}", &err_msg);
    let node = Node::Element(ElementNode {
        tag_name: s!("h2"),
        children: vec![Node::Text(err_msg)],
        ..Default::default()
    });
    return vec![node];
}

/// extracts and saves sub_templates only one level deep: children
fn extract_children_sub_templates(template_raw: &str) -> Vec<SubTemplate> {
    // drain sub-template from main template and save into vector
    // the sub_templates[0] is the main_template
    // the main template will change with draining sub-templates
    let mut sub_templates = vec![SubTemplate {
        name: s!("main_template"),
        template: s!(template_raw),
        placeholder: String::new(),
    }];

    // the syntax is <!--stmplt_field start-->, <!--stmplt_field end--> or <!--wtmplt_field start-->, <!--wtmplt_field end-->
    // unique delimiters for start and end are great if there is nesting.
    let mut pos_for_loop = 0;
    loop {
        let mut exist_template = false;
        if let Some(pos_start) = find_pos_before_delimiter(
            &sub_templates[0].template,
            pos_for_loop,
            &PREFIX.subtemplate_comment,
        ) {
            if let Some(pos_end_name) =
                find_pos_before_delimiter(&sub_templates[0].template, pos_start, " start-->")
            {
                let sub_template_name = s!(&sub_templates[0].template[pos_start + 4..pos_end_name]);
                // dbg!(sub_template_name);
                let pos_start_after_tag = pos_end_name + 9;
                let end_tag = format!("<!--{} end-->", sub_template_name);
                if let Some(pos_end_after_tag) =
                    find_pos_after_delimiter(&sub_templates[0].template, pos_start, &end_tag)
                {
                    exist_template = true;
                    // special name for template that will not be used at all.
                    // this happens when the graphic designer need more repetition of the
                    // same sub-template only for visual effect while editing.
                    if sub_template_name == "wtmplt_dummy_to_be_removed" {
                        // dbg!(pos_start);
                        // dbg!(pos_end_after_tag);
                        // remove all the template
                        sub_templates[0]
                            .template
                            .drain(pos_start..pos_end_after_tag);
                    } else {
                        let sub_template_placeholder =
                            s!(&sub_templates[0].template[pos_start..pos_start_after_tag]);
                        pos_for_loop = pos_start_after_tag;

                        // drain - extract a substring and remove it from the original
                        // leave the header with the name. It will be used
                        // as placeholder for replace later.
                        let sub_template: String = sub_templates[0]
                            .template
                            .drain(pos_start_after_tag..pos_end_after_tag)
                            .collect();
                        // remove the end tag
                        let sub_template = sub_template.trim_end_matches(&end_tag);
                        sub_templates.push(SubTemplate {
                            name: s!(sub_template_name),
                            placeholder: s!(sub_template_placeholder),
                            template: s!(sub_template),
                        });
                        // dbg!(sub_template);
                    }
                }
            }
        }
        if !exist_template {
            break;
        }
    }
    // log::info!("{:?}", sub_templates);
    // return
    sub_templates
}

// Recursive function to fill the Element with attributes
// and sub-nodes(Element, Text, Comment).
#[allow(clippy::too_many_lines, clippy::type_complexity)]
fn fill_element_node<T: HtmlTemplatingDataTrait>(
    self_0: &T,
    reader_for_microxml: &mut ReaderForMicroXml,
    mut element: ElementNode,
    html_or_svg_parent: HtmlOrSvg,
    dom_path: &mut Vec<String>,
    sub_templates: &Vec<SubTemplate>,
    subtemplate_name: &str,
    row_number: usize,
    exists_this_node: bool,
) -> Option<Result<ElementNode, &'static str>> {
    let mut replace_string: Option<String> = None;
    let mut replace_attr_name: Option<String> = None;
    let mut replace_attr_repl_name: Option<String> = None;
    let mut replace_url: Option<UrlUtf8EncodedString> = None;
    let mut replace_vec_nodes: Option<Vec<Node>> = None;
    let mut exists_next_node_or_attribute = exists_this_node;
    let mut html_or_svg_local;
    // loop through all the siblings in this iteration
    while let Some(result_token) = reader_for_microxml.next() {
        // the children inherits html_or_svg from the parent, but cannot change the parent
        html_or_svg_local = html_or_svg_parent;
        match result_token {
            Ok(token) => {
                match token {
                    Token::StartElement(tag_name) => {
                        dom_path.push(s!(tag_name));

                        // construct a child element and fill it (recursive)
                        let mut child_element = ElementNode {
                            tag_name: s!(tag_name),
                            ..Default::default()
                        };
                        if tag_name == "svg" {
                            // this tagname changes to svg now
                            html_or_svg_local = HtmlOrSvg::Svg;
                        }
                        if let HtmlOrSvg::Svg = html_or_svg_local {
                            // this is the
                            // svg elements have this namespace
                            child_element.namespace = Some(s!("http://www.w3.org/2000/svg"));
                        }
                        if tag_name == "foreignObject" {
                            // this tagname changes to html for children, not for this element
                            html_or_svg_local = HtmlOrSvg::Html;
                        }
                        // recursion
                        child_element = unwrap!(unwrap!(fill_element_node(
                            self_0,
                            reader_for_microxml,
                            child_element,
                            html_or_svg_local,
                            dom_path,
                            sub_templates,
                            subtemplate_name,
                            row_number,
                            exists_next_node_or_attribute,
                        )));

                        // ignore this node dynamic content, and don't push to result
                        // but traverse all template nodes.
                        if exists_next_node_or_attribute == true {
                            if let Some(repl_vec_nodes) = replace_vec_nodes {
                                for repl_node in repl_vec_nodes {
                                    element.children.push(repl_node);
                                }
                                replace_vec_nodes = None;
                            } else {
                                element.children.push(Node::Element(child_element));
                            }
                        }
                        // the siblings get the parents retain, until sb_ or wb_
                        exists_next_node_or_attribute = exists_this_node;
                    }
                    Token::Attribute(name, value) => {
                        if exists_this_node == true {
                            if name.starts_with(&PREFIX.attr_text) {
                                // placeholder is in the attribute name.
                                // the attribute value is only informative what is the next attribute name
                                // example: data-st_placeholder="href" href="x"
                                // The replace_string will always be applied to the next attribute. No matter the name.
                                let placeholder = name.trim_start_matches("data-");
                                let repl_txt = self_0.replace_with_string(
                                    placeholder,
                                    subtemplate_name,
                                    row_number,
                                );
                                replace_attr_name = Some(s!(value));
                                replace_attr_repl_name = Some(s!(name));
                                replace_string = Some(repl_txt);
                            } else if name.starts_with(&PREFIX.attr_url) {
                                // the same as data-st_, but exclusive to href and src
                                // because they must use an url encoded string
                                let placeholder = name.trim_start_matches("data-");
                                let repl_url = self_0.replace_with_url(
                                    placeholder,
                                    subtemplate_name,
                                    row_number,
                                );
                                replace_attr_name = Some(s!(value));
                                replace_attr_repl_name = Some(s!(name));
                                replace_url = Some(repl_url);
                            } else if name.starts_with(&PREFIX.attr_exist) {
                                // the next attribute existence
                                // if false it will not exist
                                let placeholder = name.trim_start_matches("data-");
                                let repl_bool = self_0.exists_next_node_or_attribute(placeholder);
                                exists_next_node_or_attribute = repl_bool;
                            } else if exists_next_node_or_attribute == false {
                                // don't push the next attribute
                                // usable for radio buttons checked attribute
                                // a terrible html design choice
                                exists_next_node_or_attribute = true;
                            } else {
                                // add attribute to Node
                                if let Some(repl) = replace_string {
                                    if name != &unwrap!(replace_attr_name) {
                                        panic!(
                                            "Error: Attr value of {} is not equal the next attr name {} data-model:{} dom_path: {:?} ",
                                            unwrap!(replace_attr_repl_name),
                                            name,
                                            self_0.data_model_name(),
                                            dom_path
                                        );
                                    // replace_attr_name = None;
                                    // replace_attr_repl_name=None;
                                    } else {
                                        // exclusively href and src must contain url
                                        if name == "href" || name == "src" {
                                            // error it is NOT encoded
                                            panic!(
                                                "Error: Repl of  {} name {} is NOT created as url, but as string: {}  data-model:{} dom_path: {:?}",
                                                unwrap!(replace_attr_repl_name),
                                                name,
                                                repl,
                                                self_0.data_model_name(),
                                                dom_path
                                            );
                                        } else {
                                            element.attributes.push(Attribute {
                                                name: s!(name),
                                                value: repl,
                                            });
                                        }
                                        // empty the replace_string for the next node
                                        replace_string = None;
                                        replace_attr_name = None;
                                        replace_attr_repl_name = None;
                                    }
                                } else if let Some(repl) = replace_url {
                                    if name != unwrap!(replace_attr_name.as_ref()) {
                                        panic!(
                                            "Error: Attr value of {} is not equal the next attr name {} data-model:{} dom_path: {:?} ",
                                            unwrap!(replace_attr_repl_name),
                                            name,
                                            self_0.data_model_name(),
                                            dom_path
                                        );
                                    // replace_attr_name = None;
                                    // replace_attr_repl_name = None;
                                    } else {
                                        // this is dynamic content. Must be already url encoded
                                        // from the source for "href" and "src" only.
                                        if name == "href" || name == "src" {
                                            element.attributes.push(Attribute {
                                                name: s!(name),
                                                value: s!(repl),
                                            });
                                        } else {
                                            //error. it is encoded for other attributes
                                            panic!(
                                                "Repl of {} name {} is mistakenly url encoded: {} data-model:{} dom_path: {:?}",
                                                unwrap!(replace_attr_repl_name),
                                                name,
                                                s!(repl),
                                                self_0.data_model_name(),
                                                dom_path
                                            );
                                        }
                                    }
                                    // empty the replace_string for the next node
                                    replace_url = None;
                                    replace_attr_name = None;
                                    replace_attr_repl_name = None;
                                } else {
                                    // attribute `id` is special, because it cannot be repeated in the html.
                                    // if we have many rows of data, we add a suffix with the row number in brackets like id="item(1)"
                                    // for the zero row (first row) it stays without suffix
                                    let new_value = if row_number > 0 && name == "id" {
                                        format!(
                                            "{}({})",
                                            decode_5_xml_control_characters(value),
                                            row_number
                                        )
                                    } else {
                                        // Value is coming from the template that must be well-formed.
                                        // It means that is html-encoded and we must decode it
                                        // to push it to Node where all the strings are NOT html-encoded.
                                        decode_5_xml_control_characters(value)
                                    };
                                    element.attributes.push(Attribute {
                                        name: s!(name),
                                        value: new_value,
                                    });
                                }
                            }
                        }
                    }
                    Token::TextNode(txt) => {
                        if exists_this_node == true {
                            if let Some(repl) = replace_string {
                                // empty the replace_string for the next Text node
                                replace_string = None;
                                element.children.push(Node::Text(repl));
                            } else if let Some(repl) = replace_url {
                                // empty the replace_string for the next Text node
                                replace_url = None;
                                element.children.push(Node::Text(s!(repl)));
                            } else {
                                // The template is well-formed.
                                // The string is html-encoded and must be html-decoded
                                // to push it to Node, where strings are "normal".
                                // dbg!(&dom_path);
                                // The <script> node is the exception with other rules for encoding
                                if unwrap!(dom_path.last()) == "script" {
                                    let txt = decode_html_script_node(txt);
                                    element.children.push(Node::Text(txt));
                                } else {
                                    let txt = decode_5_xml_control_characters(txt);
                                    element.children.push(Node::Text(txt));
                                }
                            };
                        }
                    }
                    Token::Comment(txt) => {
                        if exists_this_node == true {
                            // the main goal of comments is to change the value of the next text node
                            // with the result of a function
                            // it must look like <!--st_get_text--> or <!--wt_get_text-->
                            // one small exception is <textarea> because it ignores the comment syntax.
                            // It is still working, and it is not very ugly.
                            if txt.starts_with(&PREFIX.text) {
                                let repl_txt =
                                    self_0.replace_with_string(txt, subtemplate_name, row_number);
                                replace_string = Some(repl_txt);
                            } else if txt.starts_with(&PREFIX.url) {
                                let repl_url =
                                    self_0.replace_with_url(txt, subtemplate_name, row_number);
                                replace_url = Some(repl_url);
                            } else if txt.starts_with(&PREFIX.exist) {
                                // boolean if this is true than the next node exists, else doesn't exist
                                exists_next_node_or_attribute =
                                    self_0.exists_next_node_or_attribute(txt);
                            } else if txt.starts_with(&PREFIX.subtemplate) {
                                // replace exactly this placeholder for a sub-template
                                let template_name = txt.trim_end_matches(" start");
                                let repl_vec_nodes =
                                    self_0.process_sub_template(template_name, sub_templates);
                                element.children.extend_from_slice(&repl_vec_nodes[..]);
                            } else if txt.starts_with(&PREFIX.node) {
                                // nodes  (in a vector)
                                let repl_vec_nodes = self_0.replace_with_nodes(txt);
                                replace_vec_nodes = Some(repl_vec_nodes);
                            } else {
                                // it is really a comment, retain it.
                                element.children.push(Node::Comment(s!(txt)));
                            }
                        }
                    }
                    Token::EndElement(name) => {
                        let last_name = unwrap!(dom_path.pop());
                        // it can be also auto-closing element
                        if last_name == name || name == "" {
                            if name == "" {
                                element.is_self_closing = true;
                            }
                            return Some(Ok(element));
                        } else {
                            return Some(Err("End element not correct: "));
                        }
                    }
                }
            }
            Err(err_msg) => return Some(Err(err_msg)),
        }
    }
    //return
    None
}
