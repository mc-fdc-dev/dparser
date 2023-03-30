// markdown ast
#[derive(Debug)]
enum Ast {
    Detail(i32, String),
    Text(String),
}

fn main() {
    let text = "# こんにちは\nhello";
    // let mut tokens: Vec<Ast> = Vec::new();
    let tokens: Vec<Ast> = text.lines().map(|x| {
        if x.starts_with("#") {
            let mut count = 0;
            for c in x.chars() {
                if c == '#' {
                    count += 1;
                } else {
                    break;
                }
            }
            let token = Ast::Detail(count, x[((count + 1) as usize)..].to_string());
            return token;
        } else {
            let token = Ast::Text(x.to_string());
            return token;
        }
    }).collect();
    println!("{:?}", tokens);
    let mut html = String::new();
    for token in tokens {
        match token {
            Ast::Detail(level, text) => {
                html.push_str(&format!("<h{}>{}</h{}>", level, text, level));
            },
            Ast::Text(text) => {
                html.push_str(&format!("<p>{}</p>", text));
            }
        }
    }
    println!("{}", html);
    println!("Hello, world!");
}