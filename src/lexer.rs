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
    slstring = slstring.replace(" ", "");
    println!("{}", slstring);
    let chars = slstring.split("");
    for l in chars {
        println!("{}", l);
    }
    tlist
}

pub fn main() {
    let mut tokens: Vec<Token> = Vec::new();
    let html = "<html>
        </html>";
    tokenize(html, &mut tokens);
}
