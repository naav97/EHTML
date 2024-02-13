pub fn get_element_pos(tokens: &Vec<&str>, element: &str) -> usize {
    let mut re = 0;
    let mut i = 0;
    while i < tokens.len() {
        if tokens[i].starts_with(element) {
            re = i;
            break;
        }
        i = i + 1;
    }
    return re;
}

pub fn find_meta_tag(tokens: &Vec<&str>) -> usize {
    let mut re = 0;
    let mut i = 0;
    while i < tokens.len() {
        if tokens[i].starts_with("<meta") {
            if tokens[i].contains("name=\"description\"") || tokens[i].contains("name='description'") {
                re = i;
                break;
            }
        }
        i = i + 1;
    }
    return re;
}

pub fn validate_html_structure(cont: &str) -> bool {
    let mut trimmed = cont.replace("\t", "");
    trimmed = trimmed.replace("\n", "");
    trimmed = trimmed.replace("    ", "");
    trimmed = trimmed.replace("   ", "");
    trimmed = trimmed.replace(">", ">`");
    trimmed = trimmed.replace("</", "`</");
    trimmed = trimmed.replace("``", "`");
    trimmed = if trimmed.ends_with("`") { trimmed[..trimmed.len() - 1].to_string() } else { trimmed };
    let elemts: Vec<&str> = trimmed.split("`").collect();
    if !elemts[0].starts_with("<!DOCTYPE") {
        println!("ERROR: Missing or misplaced DOCTYPE tag");
        return false;
    }
    else if !elemts[1].starts_with("<html") {
        println!("ERROR: Missing or misplaced html tag");
        return false;
    }
    else if elemts[elemts.len()-1] != "</html>" {
        println!("ERROR: Code outside of html tag scope");
        return false;
    }
    else if !elemts[2].starts_with("<head") {
        println!("ERROR: Missing or misplaced head tag");
        return false;
    }
    else if elemts[elemts.len()-2] != "</body>" {
        println!("ERROR: elemnets beyond body tag scope");
        return false;
    }
    let pos_end_head = get_element_pos(&elemts, &"</head>");
    let pos_start_body = get_element_pos(&elemts, &"<body>");
    if !(pos_end_head > 2) || !(pos_end_head == pos_start_body-1) {
        println!("ERROR: head tag coses before it begins or head and body tags are not consecutive");
        return false;
    }
    let pos_start_title = get_element_pos(&elemts, &"<title");
    let pos_end_title = get_element_pos(&elemts, &"</title>");
    if !(pos_start_title > 2 && pos_start_title < pos_end_title) || !(pos_end_title < pos_end_head) {
        println!("ERROR: Missing or misplaced title tag, title tag scope null or outside of head tag scope");
        return false;
    }
    let pos_meta_desc = find_meta_tag(&elemts);
    if !(pos_meta_desc > 2 && pos_meta_desc < pos_end_head) {
        println!("ERROR: Missing or misplaced description meta tag");
        return false;
    }
    let mut i = 0;
    while i < elemts.len() {
        println!("e: {}", elemts[i]);
        i = i + 1;
    }
    true
}
