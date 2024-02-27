mod html;
mod seo;
use std::collections::HashMap;
use std::env;
use std::fs;

fn populate_vars(data: &str) -> HashMap<String, String> {
    let mut vars: HashMap<String, String> = HashMap::new();
    let data = &data.replace(">", ">`");
    let data = if data.ends_with("`") { data[..data.len() - 1].to_string() } else { data.to_string() };
    let decl: Vec<&str> = data.split("`").collect();
    for dec in decl {
        if dec.starts_with("<let ") {
            println!("{}", dec);
        }
        else {
            println!("ERROR: not a declaration ({})", dec);
        }
    }
    return vars;
}

fn main() {
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
        let parts: Vec<&str> = trimmed.split("<endvars/>").collect();
        let vars = populate_vars(parts[0]);
        if html::validate_html_structure(&parts[1]) {
            println!("HTML structure ok!");
            if seo::check_seo(&parts[1]) {
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
