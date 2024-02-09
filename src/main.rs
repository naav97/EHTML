mod html;
use std::env;
use std::fs;

fn main() {
    //lexer::main();
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("File not provided");
        return;
    }

    let file_path = &args[1];
    if let Ok(html_content) = fs::read_to_string(file_path) {
        //let dom = parse_html(&html_content);
        println!("HTML file read succesfull");
        if html::validate_html_structure(&html_content) {
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
