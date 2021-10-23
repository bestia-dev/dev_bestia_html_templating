// integration_test.rs

use dev_bestia_html_templating as tmplt;
use dev_bestia_string_utils::*;

struct DataItem01 {
    data_string: String,
    data_integer: u32,
}

struct DataList01 {
    list_name: String,
    vec_of_item: Vec<DataItem01>,
}

impl tmplt::HtmlTemplatingDataTrait for DataItem01 {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        // return
        s!("DataItem01")
    }
    /// returns a String to replace the next text-node: "wt_" or "st_"
    fn replace_with_string(
        &self,
        placeholder: &str,
        _subtemplate_name: &str,
        _pos_cursor: usize,
    ) -> String {
        // dbg!(&placeholder);
        match placeholder {
            "wt_data_string" => s!(self.data_string),
            "wt_data_integer" => s!(self.data_integer),
            _ => tmplt::utils::match_else_for_replace_with_string(
                &self.data_model_name(),
                placeholder,
            ),
        }
    }
}

#[test]
fn integration_test_01() {
    let data_item_01 = DataItem01 {
        data_string: s!("replaced_01"),
        data_integer: 33,
    };

    let html_template = r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <meta http-equiv="Content-type" content="text/html; charset=utf-8" />
        <title>test_01</title>
        <meta name="Description" content="test_01" />
        <link rel="stylesheet" href="css/cargo_crev_reviews.css" />
    </head>
    <body>
        <div class="container_0">
            <h2>test_01</h2>
            <p><!--wt_data_string-->sample_1</p>
            <p><!--wt_data_integer-->00</p>
        </div>
        <div class="container_0">
            <p>Fixed text node.</p>
        </div>
    </body>
</html>
    "#;

    let new_html = tmplt::process_html(&data_item_01, &html_template);

    // println!("{}", new_html);

    assert_eq!(
        new_html,
        r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <meta http-equiv="Content-type" content="text/html; charset=utf-8" />
        <title>test_01</title>
        <meta name="Description" content="test_01" />
        <link rel="stylesheet" href="css/cargo_crev_reviews.css" />
    </head>
    <body>
        <div class="container_0">
            <h2>test_01</h2>
            <p>replaced_01</p>
            <p>33</p>
        </div>
        <div class="container_0">
            <p>Fixed text node.</p>
        </div>
    </body>
</html>"#
    );
}
