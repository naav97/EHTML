mod html;
use std::env;
use std::fs;

fn check_title_lenght(cont: &str) -> bool {
    let mut trimmed = cont.replace("<title", "`<title");
    trimmed = trimmed.replace("</title>", "</title>`");
    let mut tokens: Vec<&str> = trimmed.split("`").collect();
    tokens.retain(|&tok| tok.starts_with("<title"));
    if tokens.len() > 1 {
        println!("ERROR: more than 1 title tag");
        return false;
    }
    let mut tit_cont = tokens[0].replace("<title>", "");
    tit_cont = tit_cont.replace("</title>", "");
    return tit_cont.chars().count() >= 50 && tit_cont.chars().count() <= 60;
}

fn check_seo(cont: &str) -> bool {
    if !check_title_lenght(cont) {
        println!("WARNING: title is to long or to short, or theres an error");
        return false;
    }
    //let mut trimmed = cont.replace("<image ", "`<image ");
    //trimmed = trimmed.replace(">", ">`");
    //trimmed = trimmed.replace("<meta ", "`<meta ");
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
        println!("HTML file read succesfull");
        let mut trimmed = html_content.replace("\t", "");
        trimmed = trimmed.replace("\n", "");
        if html::validate_html_structure(&trimmed) {
            println!("HTML structure ok!");
            if check_seo(&trimmed) {
                println!("SEO ok!");
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
