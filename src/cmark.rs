
//#[link(name="cmark-gfmextensions")]
//#[link(name="cmark-gfm")]
//#[link(name="cmark", kind="static")]
#[link(name="cmark-wrapper")]
extern "C" {
    // Our C function definitions!
    pub fn markdown_to_html(src: *const u8, dst:*mut *mut u8) -> u32;
    pub fn markdown_free(s: *mut u8);
}

pub fn to_html(markdown: &str) -> Result<String, &str> {
    //let mut html = Err("invalid markdown text");
    let m = markdown.as_bytes();
    unsafe {
        let mut x:u8 = 0;
        let mut xx:*mut u8 = &mut x; 
        let mut dst :*mut *mut u8 = &mut xx;
        let mut md = markdown_to_html(m.as_ptr(), dst);
        let html = String::from_raw_parts(*dst, md as usize,md as usize);
        markdown_free(*dst);
        Ok(html)
    }
    //html
}
