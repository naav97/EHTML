mod html;
mod seo;
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
        println!("HTML file read succesfull");
        let mut trimmed = html_content.replace("\t", "");
        trimmed = trimmed.replace("\n", "");
        if html::validate_html_structure(&trimmed) {
            println!("HTML structure ok!");
            if seo::check_seo(&trimmed) {
                println!("SEO ok!");
            }
            else {
                println!("SEO can be improved");
            }
        }
        else {
            println!("somehting wrong width HTML structure");
        }
    }
    else {
        println!("Error reading HTML file");
    }
}
