mod html;
mod seo;
mod engine;
use crate::engine::populate_vars;
use crate::engine::replace_vars;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("File not provided");
        return;
    }

    let file_path = &args[1];
    if let Ok(html_content) = fs::read_to_string(file_path) {
        println!("HTML file read succesfull");
        let parts: Vec<&str> = html_content.split("<endvars/>").collect();
        let mut trimmed_vars = parts[0].replace("\t", "");
        trimmed_vars = trimmed_vars.replace("\n", "");
        let vars = populate_vars(&trimmed_vars);
        let final_html: String = replace_vars(vars, &parts[1]);
        println!("{}", final_html);
        let mut trimmed_html = final_html.replace("\t", "");
        trimmed_html = trimmed_html.replace("\n", "");
        if html::validate_html_structure(&trimmed_html) {
            println!("HTML structure ok!");
            if seo::check_seo(&trimmed_html) {
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
