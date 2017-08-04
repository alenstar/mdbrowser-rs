mod cmark;

fn main() {
    let md = "# Hello\n### world !\n";
    let html = cmark::to_html(md);
    println!("{}", html.unwrap());
}
