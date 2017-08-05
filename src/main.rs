mod cmark;

fn main() {
    let md = "# Hello\n### world !\n";
    let base_css = include_str!("../static/base.css");
    let base_html = include_str!("../static/base.html");
    let html = cmark::HtmlBody::new_from_markdown(md);
    if !html.as_string().is_empty() {
    	let result = str::replace(base_html, "{%style%}", base_css);
    	let result = str::replace(&result, "{%body%}", &html);
    	println!("{}", result);
	} else {
		println!("markdown convert to html failed");
	}
}
