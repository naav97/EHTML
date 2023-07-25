enum TokenType {
    DoctypeToken,
    HtmlOpenTag,
    HtmlCloseTag,
}

struct Token {
    value: String,
    kind: TokenType,
}

fn tokenize<'a>(source: &str, tlist: &'a mut Vec<Token>) -> &'a mut Vec<Token> {
    let mut slstring = source.replace("\n", "");
    slstring = slstring.replace("\t", "");
    slstring = slstring.replace("    ", "");
    slstring = slstring.replace("<", " < ");
    slstring = slstring.replace(">", " > ");
    slstring = slstring.replace("!", " ! ");
    slstring = slstring.replace("=", " = ");
    slstring = slstring.replace("/", " / ");

    slstring = slstring.replace("  ", " ");

    let chars = slstring.split(" ");
    let vchar: Vec<&str> = chars.collect();
    let mut i = 0;
    while i < vchar.len() {
        if vchar[i] == "<" {
            if vchar[i+1] == "/" {
                println!("close tag!");
            }
            else if vchar[i+1] == "!" {
                if vchar[i+2] == "DOCTYPE" {
                    println!("DOCTYPE");
                }
                else if vchar[i+2].starts_with("-") {
                    println!("comment");
                }
                else {
                    println!("ERROR: Whas expecting DOCTYPE tag or comment but got {}", vchar[i+2]);
                    break;
                }
            }
            else {
                println!("open tag!");
            }
        }
        i = i + 1;
    }
    tlist
}

pub fn main() {
    let mut tokens: Vec<Token> = Vec::new();
    let html = "<!DOCTYPE> <!-- doctye --> <!fregrw
        <html>
        </html>";
    tokenize(html, &mut tokens);
}
