mod cmark;

//extern crate glib;
extern crate gtk;
extern crate webkit2gtk;

use gtk::prelude::*;
#[cfg(feature="v2_4")]
use glib::ToVariant;
use gtk::{ContainerExt, Inhibit, WidgetExt, Window, WindowExt, WindowType,WindowPosition, AboutDialog, CheckMenuItem, IconSize, Image, Label, Menu, MenuBar, MenuItem,};
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
    //window.add(&webview);

    let settings = WebView::get_settings(&webview).unwrap();
    settings.set_enable_developer_extras(true);

    /*let inspector = webview.get_inspector().unwrap();
    inspector.show();*/

    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 4);
    let menu = Menu::new();
    let menu_bar = MenuBar::new();
    let file = MenuItem::new_with_label("File");
    let about = MenuItem::new_with_label("About");
    let quit = MenuItem::new_with_label("Quit");
    let file_item = MenuItem::new();
    let file_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let file_image = Image::new_from_file("resources/file.png");
    let file_label = Label::new(Some("File"));
    let folder_item = MenuItem::new();
    let folder_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let folder_image = Image::new_from_icon_name("folder-music-symbolic", IconSize::Menu.into());
    let folder_label = Label::new(Some("Folder"));
    let check_item = CheckMenuItem::new_with_label("Click me!");

    file_box.pack_start(&file_image, false, false, 0);
    file_box.pack_start(&file_label, true, true, 0);
    file_item.add(&file_box);
    folder_box.pack_start(&folder_image, false, false, 0);
    folder_box.pack_start(&folder_label, true, true, 0);
    folder_item.add(&folder_box);
    menu.append(&file_item);
    menu.append(&folder_item);
    menu.append(&check_item);
    menu.append(&about);
    menu.append(&quit);
    file.set_submenu(Some(&menu));
    menu_bar.append(&file);

    let other_menu = Menu::new();
    let sub_other_menu = Menu::new();
    let other = MenuItem::new_with_label("Another");
    let sub_other = MenuItem::new_with_label("Sub another");
    let sub_other2 = MenuItem::new_with_label("Sub another 2");
    let sub_sub_other2 = MenuItem::new_with_label("Sub sub another 2");
    let sub_sub_other2_2 = MenuItem::new_with_label("Sub sub another2 2");

    sub_other_menu.append(&sub_sub_other2);
    sub_other_menu.append(&sub_sub_other2_2);
    sub_other2.set_submenu(Some(&sub_other_menu));
    other_menu.append(&sub_other);
    other_menu.append(&sub_other2);
    other.set_submenu(Some(&other_menu));
    // menu_bar.append(&other);

    let edit_menu = Menu::new();
    let edit = MenuItem::new_with_label("Edit");
    let edit_preview = CheckMenuItem::new_with_label("Preview"); //("实时预览");

    edit_menu.append(&edit_preview);
    edit.set_submenu(Some(&edit_menu));
    menu_bar.append(&edit);

    let view_menu = Menu::new();
    let view = MenuItem::new_with_label("View");
    let view_preview = CheckMenuItem::new_with_label("Preview"); //("实时预览");

    view_menu.append(&view_preview);
    view.set_submenu(Some(&view_menu));
    menu_bar.append(&view);

    v_box.pack_start(&menu_bar, false, false, 0);
    v_box.pack_start(&webview, true, true, 0);
    window.add(&v_box);

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    about.connect_activate(move |_| {
        let p = AboutDialog::new();
        p.set_authors(&["gtk-rs developers"]);
        p.set_website_label(Some("gtk-rs"));
        p.set_website(Some("http://gtk-rs.org"));
        p.set_authors(&["Gtk-rs developers"]);
        p.set_title("About!");
        p.set_transient_for(Some(&window));
        p.run();
        p.destroy();
    });
    check_item.connect_toggled(|w| {
        /*
        w.set_label(if w.get_active() {
            "Checked"
        } else {
            "Unchecked"
        });
        */
        if w.get_active() {
            println!("Checked");
        } else {
            println!("Unchecked");
        }
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
