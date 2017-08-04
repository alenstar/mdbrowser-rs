
//#[link(name="cmark-gfmextensions")]
//#[link(name="cmark-gfm")]
//#[link(name="cmark", kind="static")]
#[link(name="cmark-wrapper")]
extern "C" {
    // Our C function definitions!
    pub fn markdown_to_html(src: *const u8) -> *mut u8;
    pub fn markdown_free(s: *mut u8);
}

pub fn to_html(markdown: &str) -> Option<String> {
    let mut html = String::new();
    let m = markdown.as_bytes();
    unsafe {
        let mut md = markdown_to_html(m.as_ptr());
        // TODO
        // ...
        //
        markdown_free(md);
    }
    // "html".to_string()
    None
}
