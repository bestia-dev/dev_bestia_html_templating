// integration_test.rs

struct DataItem01{
    data_string:String,
    data_integer:u32
}

struct DataList01{
    list_name:String,
    vec_of_item: vec<DataItem01>,
}

impl HtmlServerTemplateRender for DataItem01 {
    /// data model name is used for eprint
    fn data_model_name(&self) -> String {
        // return
        s!("DataItem01")
    }
    /// renders the complete html file.
    fn render_html(&self, html: &str) -> String {
        let html = render(self, html, ServerOrClient::WebBrowserClient);
        // return
        html
    }
    /// returns a String to replace the next text-node: "wt_" or "st_"
    fn replace_with_string(&self, placeholder: &str, _subtemplate_name: &str, _pos_cursor: usize) -> String {
        // dbg!(&placeholder);
        match placeholder {
            "wt_data_string" => self.data_string.to_string(),
            "wt_data_integer" => self.data_integer.to_string(),
            _ => replace_with_string_match_else(&self.data_model_name(), placeholder),
        }
    }
}

#[test]
fn test_01() {

    let data_item_01 = DataItem01{
        data_string="replaced_01".to_string(),
        data_integer = 33,
    };

    let html_template = r#"
<!DOCTYPE html>
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

    data_item_01.render_html(html_template);
    //assert_eq!(adder::add(3, 2), 5);
}