// html_templating_data_trait_mod.rs

//! html templating library for html
//! It can work on the server and on the client (web browser in wasm).
//! it reads through every token in the html and when finds a template variable
//! example: <!--wt_name--> or data-wt_name="class" or <!--wb_exist_next-->
//! it calculates it and replace the next token: it can be a text_node, attribute, a complex node,...
//! On the server the placeholders start with "s" like "st_name" : st_, sb_,...
//! On the web web browser (client) the placeholders start with "w" like "wt_name": wt_, wb,...
//! It is possible to use partially on the server and partially on the client.
//! It is compatible also with `svg` where special namespaces are used.

// region: use
use crate::trait_utils_mod;

use dev_bestia_string_utils::*;
use dev_bestia_url_utf8::*;

// endregion: use

#[derive(Clone, Debug, Default)]
pub struct ElementNode {
    pub tag_name: String,
    pub attributes: Vec<Attribute>,
    pub children: Vec<trait_utils_mod::Node>,
    pub namespace: Option<String>,
    pub is_self_closing: bool,
}
/// An attribute on a DOM node, such as `id="my-thing"`
#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: String,
    /// attribute value. The text must be not encoded.
    /// It will be xml encoded when converting the node to html string.
    pub value: String,
}

/// The same html templating can be used on the server or on the client
/// but if we want to mix it, we need to distinguish template variables with different prefixes
/// example: "st_" for server or "wt_" for client (web browser)
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum ServerOrClient {
    // server
    Server,
    /// web browser client
    WebBrowserClient,
}

#[derive(Clone, Debug)]
pub struct PrefixForTemplateVariables {
    pub text: String,
    pub url: String,
    pub exist: String,
    pub node: String,
    pub subtemplate: String,
    pub subtemplate_comment: String,
    pub attr_text: String,
    pub attr_url: String,
    pub attr_exist: String,
}

/// the trait has only methods that can be implemented (overridden)
/// It is not possible to define a method as "override forbidden"
/// because the data model is always different and is known only to the project.
/// if not implemented, the default functions below will be used.
pub trait HtmlTemplatingDataTrait {
    /// name of data model for debugging
    fn data_model_name(&self) -> String;
    /// returns a String to replace the next text-node or attribute value
    /// use macro s!() for a normal string
    fn replace_with_string(
        &self,
        placeholder: &str,
        subtemplate_name: &str,
        row_number: usize,
    ) -> String;
    /// same as replace_with_string, but return url
    /// exclusively for attributes value of href and src
    /// the url must be encoded in the beginning because it encodes segments of
    /// url prior to being composed together.
    /// use macro url_u!() to create an url, very like format!
    /// I try to avoid String here to force the developer to not forget to url_encode
    fn replace_with_url(
        &self,
        placeholder: &str,
        _subtemplate_name: &str,
        _pos_cursor: usize,
    ) -> UrlUtf8EncodedString {
        // dbg!( &placeholder);
        match placeholder {
            _ => trait_utils_mod::match_else_for_replace_with_url(
                &self.data_model_name(),
                placeholder,
            ),
        }
    }
    /// boolean : is the next node existing or not: "wb_" or "sb_"
    fn exists_next_node_or_attribute(&self, placeholder: &str) -> bool {
        // dbg!( &placeholder);
        match placeholder {
            _ => trait_utils_mod::match_else_for_exists_next_node_or_attribute(
                &self.data_model_name(),
                placeholder,
            ),
        }
    }
    /// returns a vector of Nodes to replace the next Node
    fn replace_with_nodes(&self, placeholder: &str) -> Vec<trait_utils_mod::Node> {
        // dbg!(&placeholder);
        match placeholder {
            _ => trait_utils_mod::match_else_for_replace_with_nodes(
                &self.data_model_name(),
                placeholder,
            ),
        }
    }
    /// process sub-template
    fn process_sub_template(
        &self,
        template_name: &str,
        _sub_templates: &Vec<trait_utils_mod::SubTemplate>,
    ) -> Vec<trait_utils_mod::Node> {
        log::info!("{}", template_name);
        match template_name {
            _ => trait_utils_mod::match_else_for_process_sub_template(
                &self.data_model_name(),
                template_name,
            ),
        }
    }
}
// region: methods/functions that must not be overridden
// this is why they cannot be inside the trait
// and I must use a generic parameter <T> instead of simply "self"

// endregion: methods/functions that must not be overridden

// endregion: ? private functions for trait HtmlTemplatingDataTrait

// region: utility fn

/// in html the <script> element is encoded differently
pub fn encode_html_script_node(input: &str) -> String {
    input.replace("</script>", "\\x3c/script>")
}

/// in html the <script> element is decoded differently
pub fn decode_html_script_node(input: &str) -> String {
    input.replace("\\x3c/script>", "</script>")
}

/// private fn - decode 5 xml control characters : " ' & < >
/// <https://www.liquid-technologies.com/XML/EscapingData.aspx>
/// I will ignore all html entities, to keep things simple,
/// because all others characters can be written as utf-8 characters.
/// it is mandatory that text is valid utf-8.
/// <https://www.tutorialspoint.com/html5/html5_entities.htm>
/// TODO: find a faster method // The standard library replace() function makes allocation,
pub fn decode_5_xml_control_characters(input: &str) -> String {
    input
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}

/// TODO: find a faster method // The standard library replace() function makes allocation,
/// Just to talk about XSS attack on attribute value.
/// let name = "dummy onmouseover=alert(/XSS/)";    // User input
/// let tag = format!("<option value={}>", htmlescape::encode_minimal(name));
/// // Here `tag` is    "<option value=dummy onmouseover=alert(/XSS/)>"
/// I use templates that must be microxml compatible.
/// There cannot exist an attribute value without quotes.
pub fn encode_5_xml_control_characters(input: &str) -> String {
    input
        .replace("&", "&amp;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
}

/// to string, but zero converts to empty
#[allow(dead_code)]
fn url_s_zero_to_empty(number: usize) -> String {
    if number == 0 {
        s!()
    } else {
        s!(number)
    }
}
// endregion: utility fn
