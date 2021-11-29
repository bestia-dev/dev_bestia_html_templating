[comment]: # (auto_md_to_doc_comments segment start A)

# dev_bestia_html_templating

[comment]: # (auto_cargo_toml_to_md start)

**Templating library for html in fullstack Rust, server-side or client-side in wasm**  
***[repository](https://github.com/lucianobestia/dev_bestia_html_templating); version: 0.1.41  date: 2021-11-29 authors: Luciano Bestia***  

[comment]: # (auto_cargo_toml_to_md end)

[comment]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-749-green.svg)](https://github.com/LucianoBestia/dev_bestia_html_templating/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-118-blue.svg)](https://github.com/LucianoBestia/dev_bestia_html_templating/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-134-purple.svg)](https://github.com/LucianoBestia/dev_bestia_html_templating/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/dev_bestia_html_templating/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-95-orange.svg)](https://github.com/LucianoBestia/dev_bestia_html_templating/)

[comment]: # (auto_lines_of_code end)

[comment]: # (auto_badges start)

[![crates.io](https://img.shields.io/crates/v/dev_bestia_html_templating.svg)](https://crates.io/crates/dev_bestia_html_templating) [![Documentation](https://docs.rs/dev_bestia_html_templating/badge.svg)](https://docs.rs/dev_bestia_html_templating/) [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/dev_bestia_html_templating.svg)](https://web.crev.dev/rust-reviews/crate/dev_bestia_html_templating/) [![Lib.rs](https://img.shields.io/badge/Lib.rs-rust-orange.svg)](https://lib.rs/crates/dev_bestia_html_templating/) [![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/LucianoBestia/dev_bestia_html_templating/blob/master/LICENSE) [![Rust](https://github.com/LucianoBestia/dev_bestia_html_templating/workflows/RustAction/badge.svg)](https://github.com/LucianoBestia/dev_bestia_html_templating/actions)  

[comment]: # (auto_badges end)

## Try it

## Motivation

Internet browsers are available practically on all platforms. They are based today on the `W3C` standards  `html5` and `css3`. All modern browsers support also wasm/webassembly (instead or beside javascript). Rust is a fantastic language to be compiled into wasm. This combination can be used as a cross-platform GUI.  
A program user interface is made of data and presentation. Sadly html has this all mixed in one "file". To separate data from presentation we use a templating library.  
First a graphical designer creates a `html5 + css3` presentation with some sample texts. When he is satisfied, then the programmer puts some comments and attributes inside the html file. These markers are processed by the templating library and the data is injected into the html. So we start with a clear separation of data and presentation and end with a normal html file.  

### Create a "standard" html page

The html page has to be MicroXml compatible, basically XHtml.  Copy for example `web_server_folder/review_list.html` to a new html file. Open this file with a browser to preview it. I use the VSCode extension [vscode-open-wsl](https://marketplace.visualstudio.com/items?itemName=NoThlnG.vscode-open-wsl) and right-click on the file and `Open with default application`. In WSL2 I use my project [wsl_open_browser](https://github.com/LucianoBestia/wsl_open_browser). Now edit the html file to your liking and reload the browser with F5 to see the changes. Use some sample text to make it look as close to what you want. These texts will be later programmatically replaced, but they are invaluable while designing a nice layout.

### Add markers

Inside the html you want to replace the sample texts with the data from the server. Before the text add the (invisible) marker for example `<!--wt_crate_name-->`. You can replace also an attribute if you insert an attribute before it like this `data-wt_variable_name="next_attribute_name"`.  
Now run the automation task `cargo auto build` that will copy/embed this file into `files_mod.rs`.  

## cargo crev reviews and advisory

We live in times of danger with [supply chain attacks](https://en.wikipedia.org/wiki/Supply_chain_attack).  
It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
You can also read reviews quickly on the web:  
<https://web.crev.dev/rust-reviews/crates/>  

## open-source free and free as a beer

My open-source projects are free and free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful,  
please buy me a beer donating on my [paypal](https://www.paypal.com/paypalme/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[comment]: # (auto_md_to_doc_comments segment end A)
