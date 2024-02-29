mod html;
mod seo;
use std::collections::HashMap;
use std::env;
use std::fs;
use regex::Regex;

fn proc_val(val: &str) -> String {
    if val.contains("\"") { //|| val.contains("'")
        let mut temp = val.to_string();
        while true {
            if temp.starts_with(" ") {
                temp.remove(0);
            }
            else if temp.starts_with("\"") {
                temp.remove(0);
                break;
            }
            else {
                panic!("ERROR: bad format in string");
            }
        }
        while true {
            if temp.ends_with(" ") {
                temp.remove(temp.len() - 1);
            }
            else if temp.ends_with("\"") {
                temp.remove(temp.len() - 1);
                break;
            }
            else {
                panic!("ERROR: bad format in string");
            }
        }
        return temp;
    }
    else {
        return val.replace(" ", "").to_string();
    }
}

fn proc_unty_var(def: &str, hmap: &mut HashMap<String, String>) {
    let name = &def.split("=").collect::<Vec<&str>>()[0].replace(" ", "");
    let val = &def.split("=").collect::<Vec<&str>>()[1].replace(">", "");
    let trimval = proc_val(&val);
    hmap.insert(name.to_string(), trimval);
}

fn proc_typed_var(def: &str, hmap: &mut HashMap<String, String>) {
    let name = &def.split(":").collect::<Vec<&str>>()[0].replace(" ", "");
    if def.contains("=") {
        let vtype = &def.split(":").collect::<Vec<&str>>()[1].split("=").collect::<Vec<&str>>()[0].replace(" ", "");
        let val = &def.split(":").collect::<Vec<&str>>()[1].split("=").collect::<Vec<&str>>()[1].replace(">", "");
        let trimval = proc_val(&val);
        if vtype == "int" {
            let regex = Regex::new(r"^-?\d+$").expect("type missmatch error");
            if !regex.is_match(&trimval) {
                panic!("ERROR: type missmatch expected int but got {}", trimval);
            }
            hmap.insert(name.to_string(), trimval);
        }
        else if vtype == "float" {
            let regex = Regex::new(r"^-?\d+(\.\d+)?$").expect("type missmatch error");
            if !regex.is_match(&trimval) {
                panic!("ERROR: type missmatch expected float but got {}", trimval);
            }
            hmap.insert(name.to_string(), trimval);
        }
        else if vtype == "bool" {
            let regex = Regex::new(r"^(true|false)$").expect("type missmatch error");
            if !regex.is_match(&trimval) {
                panic!("ERROR: type missmatch expected bool but got {}", trimval);
            }
            hmap.insert(name.to_string(), trimval);
        }
        else if vtype == "char" {
            let regex = Regex::new(r"^.{1}$").expect("type missmatch error");
            if !regex.is_match(&trimval) {
                panic!("ERROR: type missmatch expected char but got {}", trimval);
            }
            hmap.insert(name.to_string(), trimval);
        }
        else if vtype == "str" {
            let regex = Regex::new(r"^.+$").expect("type missmatch error");
            if !regex.is_match(&trimval) {
                panic!("ERROR: type missmatch expected str but got {}", trimval);
            }
            hmap.insert(name.to_string(), trimval);
        }
        else {
            panic!("ERROR: unknown variable type {}", vtype);
        }
    }
    else {
        panic!("ERROR: missing '=' in {}", def);
    }
}

fn populate_vars(data: &str) -> HashMap<String, String> {
    let mut vars: HashMap<String, String> = HashMap::new();
    let data = &data.replace(">", ">`");
    let data = if data.ends_with("`") { data[..data.len() - 1].to_string() } else { data.to_string() };
    let decl: Vec<&str> = data.split("`").collect();
    for dec in decl {
        if dec.starts_with("<let ") {
            let def = dec.replace("<let ", "");
            if def.contains(":") {
                proc_typed_var(&def, &mut vars);
            }
            else if def.contains("=") {
                proc_unty_var(&def, &mut vars);
            }
            else {
                panic!("ERROR: invalid declaration: {} missing '=' or ':'", def);
            }
        }
        else {
            panic!("ERROR: not a declaration ({})", dec);
        }
    }
    for (name, val) in &vars {
        println!("{name}: {val}");
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
