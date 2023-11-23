#[macro_use]
extern crate html5ever;
extern crate markup5ever_rcdom as rcdom;

//mod lexer;
use std::env;
use std::fs;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use markup5ever_rcdom::{Handle, NodeData, RcDom};

fn main() {
    //lexer::main();
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("File not provided");
        return;
    }

    let file_path = &args[1];
    println!("Arg: {}", file_path);
}
