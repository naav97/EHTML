#[macro_use]
extern crate html5ever;
extern crate markup5ever_rcdom as rcdom;

//mod lexer;
use std::env;
use std::fs;
use html5ever::ParseOpts;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use markup5ever_rcdom::{Handle, NodeData, RcDom};

fn parse_html(html: &str) -> RcDom {
    let opts = TreeBuilderOpts {
        scripting_enabled: false,
        ..Default::default()
    };
    let opts_comp = ParseOpts {
        tree_builder: opts,
        ..Default::default()
    };
    html5ever::parse_document(RcDom::default(), opts_comp).from_utf8().read_from(&mut html.as_bytes()).unwrap()
}

fn validate_html_structure(dom: &RcDom) -> bool {
    if let NodeData::Doctype { .. } = dom.document.children.borrow()[0].data {
        println!("DOCTYPE ok!");
    }
    else {
        return false;
    }
    //if let NodeData::Element { name, .. } = &node.data {
    //    // Check if the element name is "h1"
    //    return name.local.as_ref() == "h1";
    //}
    //let html_element = &dom.document.children.borrow()[1];
    //let html_ok = html_element.data.name.local.as_ref() == "html";

    true
}

fn main() {
    //lexer::main();
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("File not provided");
        return;
    }

    let file_path = &args[1];
    if let Ok(html_content) = fs::read_to_string(file_path) {
        let dom = parse_html(&html_content);
        println!("HTML file read succesfull");
        if validate_html_structure(&dom) {
            println!("HTML structure ok!");
        }
        else {
            println!("somehting wrong width HTML structure");
        }
    }
    else {
        println!("Error reading HTML file");
    }
}
