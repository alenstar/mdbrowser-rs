mod cmark;

//extern crate glib;
extern crate gtk;
extern crate webkit2gtk;

#[cfg(feature="v2_4")]
use glib::ToVariant;
use gtk::{ContainerExt, Inhibit, WidgetExt, Window, WindowExt, WindowType};
use webkit2gtk::{WebContext, WebView, WebViewExtManual};
#[cfg(feature="v2_6")]
use webkit2gtk::UserContentManager;

fn main() {
    gtk::init().unwrap();

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Markdown Browser");
    window.set_default_size(800, 600);

    let context = WebContext::get_default().unwrap();
    #[cfg(feature="v2_4")]
    context.set_web_extensions_initialization_user_data(&"webkit".to_variant());
    context.set_web_extensions_directory("../webkit2gtk-webextension-rs/example/target/debug/");
    #[cfg(feature="v2_6")]
    let webview = WebView::new_with_context_and_user_content_manager(&context, &UserContentManager::new());
    #[cfg(not(feature="v2_6"))]
    let webview = WebView::new_with_context(&context);
    webview.load_uri("https://baidu.com/");
    window.add(&webview);

    let settings = WebView::get_settings(&webview).unwrap();
    settings.set_enable_developer_extras(true);

    /*let inspector = webview.get_inspector().unwrap();
    inspector.show();*/

    window.show_all();
/*
    webview.run_javascript("alert('Hello');");
    webview.run_javascript_with_callback("42", |result| {
        match result {
            Ok(result) => {
                let context = result.get_global_context().unwrap();
                let value = result.get_value().unwrap();
                println!("is_boolean: {}", value.is_boolean(&context));
                println!("is_number: {}", value.is_number(&context));
                println!("{:?}", value.to_number(&context));
                println!("{:?}", value.to_boolean(&context));
            },
            Err(error) => println!("{}", error),
        }
    });
*/
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

	gtk::main();

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
