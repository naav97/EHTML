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

fn check_images(cont: &str) -> bool {
    let mut trimmed = cont.replace("<img ", "`<img ");
    trimmed = trimmed.replace(">", ">`");
    let mut tokens: Vec<&str> = trimmed.split("`").collect();
    tokens.retain(|&tok| tok.starts_with("<img "));
    let mut ret = true;
    for imgE in tokens {
        if let Some(src_init) = imgE.find("src=") {
            let init = src_init + "src=\"".len();
            let mut src = &imgE[init..];
            if let Some(end_img) = src.find("\"") {
                src = &src[..end_img];
                let src_parts: Vec<&str> = src.split(".").collect();
                let ext = src_parts[src_parts.len()-1];
                if ext == "bmp" || ext == "ico" || ext == "cur" || ext == "tif" || ext == "tiff" {
                    println!("WARNING: consider using a diferent format than {} in the image {} like PNG or WebP", ext, imgE);
                    ret = false;
                }
            }
        }
        else {
            println!("ERROR: the following img has no src atribute: {}", imgE);
            ret = false;
        }
        if let Some(alt_init) = imgE.find("alt=") {
            let init = alt_init + "alt=\"".len();
            let alt = &imgE[init..];
            if alt.chars().nth(0).unwrap() == '"' || alt.chars().nth(1).unwrap() == '"' {
                println!("WARNING: empty or too short alt atribute in: {}", imgE);
                ret = false;
            }
        }
        else {
            println!("WARNING: the following img has no alt atribute: {}", imgE);
            ret = false;
        }
        if !imgE.contains("loading=\"lazy\"") {
            println!("WARNING: consider lazy loading the following img: {}", imgE);
        }
    }
    ret
}

pub fn check_seo(cont: &str) -> bool {
    let mut re: bool = true;
    if !check_title_length(cont) {
        println!("WARNING: title is too long or too short, or theres an error");
        re = false;
    }
    if !check_desc_length(cont) {
        println!("WARNING: page description too long or too shrot");
        re = false;
    }
    if !check_images(cont) {
        re = false;
    }
    re
}
