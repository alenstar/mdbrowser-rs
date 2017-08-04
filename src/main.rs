mod cmark;

fn main() {
    let md = "# Hello\n### world !\n";
    let html = cmark::HtmlBody::new_from_markdown(md);
    println!("{}", html.as_string());
}
