mod html;
use std::env;
use std::fs;

fn check_title_length(cont: &str) -> bool {
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

fn check_desc_length(cont: &str) -> bool {
    let mut trimmed = cont.replace("<meta", "`<meta");
    trimmed = trimmed.replace(">", ">`");
    let mut tokens: Vec<&str> = trimmed.split("`").collect();
    tokens.retain(|&tok| tok.starts_with("<meta") && tok.contains("name=\"description\""));
    if let Some(cont_pos) = tokens[0].find("content=") {
        let init = cont_pos + "content=\"".len();
        tokens[0] = &tokens[0][init..];
        if let Some(end_pos) = tokens[0].find("\"") {
            tokens[0] = &tokens[0][..end_pos];
            return tokens[0].len() >= 150 && tokens[0].len() <= 160;
        }
        else {
            println!("ERROR: something went wrong");
            return false;
        }
    }
    else {
        println!("ERROR: missing content value in description meta tag");
        return false;
    }
}

fn check_seo(cont: &str) -> bool {
    let mut re: bool = true;
    if !check_title_length(cont) {
        println!("WARNING: title is to long or to short, or theres an error");
        re = false;
    }
    if !check_desc_length(cont) {
        println!("WARNING: page description to long or to shrot");
        re = false;
    }
    //let mut trimmed = cont.replace("<image ", "`<image ");
    //trimmed = trimmed.replace(">", ">`");
    //trimmed = trimmed.replace("<meta ", "`<meta ");
    re
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
