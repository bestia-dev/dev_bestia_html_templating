// dev_bestia_html_templating lib.rs

#![doc=include_str!("../README.md")]

// internal private modules
mod html_processor_mod;
mod html_templating_data_trait_mod;
mod trait_utils_mod;
mod utils_mod;

// region: public interface

// export/re-export public functions, traits,...
pub use html_processor_mod::process_html;
pub use html_templating_data_trait_mod::HtmlTemplatingDataTrait;

// public module exports/re-export functions, enum, structs,...
pub mod utils {
    pub use crate::trait_utils_mod::match_else_for_exists_next_node_or_attribute;
    pub use crate::trait_utils_mod::match_else_for_process_sub_template;
    pub use crate::trait_utils_mod::match_else_for_replace_with_nodes;
    pub use crate::trait_utils_mod::match_else_for_replace_with_string;
    pub use crate::trait_utils_mod::match_else_for_replace_with_url;
    pub use crate::trait_utils_mod::process_template_raw_to_nodes;
    pub use crate::trait_utils_mod::HtmlOrSvg;
    pub use crate::trait_utils_mod::Node;
    pub use crate::trait_utils_mod::SubTemplate;
}

// endregion: public interface
