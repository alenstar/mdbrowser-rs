use std::ops::Drop;
use std::ops::Deref;

#[link(name="cmark-wrapper")]
extern "C" {
    // Our C function definitions!
    pub fn markdown_to_html(src: *const u8, dst: *mut *mut u8) -> u32;
    pub fn markdown_free(s: *mut u8);
}

pub struct HtmlBody {
    raw_body: Option<*mut u8>,
    body: String,
}
impl HtmlBody {
    pub fn new() -> HtmlBody {
        HtmlBody {
            raw_body: None,
            body: String::new(),
        }
    }

    pub fn new_from_markdown(markdown: &str) -> HtmlBody {

        let html = HtmlBody::_load_markdown(markdown);
        match html {
            Some(x) => unsafe {
                HtmlBody {
                    raw_body: Some(x.0),
                    body: String::from_raw_parts(x.0, x.1, x.1),
                }
            },
            _ => HtmlBody::new(),
        }
    }

    fn _load_markdown(markdown: &str) -> Option<(*mut u8, usize)> {
        let m = markdown.as_bytes();
        unsafe {
            let mut x: u8 = 0;
            let mut xx: *mut u8 = &mut x;
            let mut dst: *mut *mut u8 = &mut xx;
            let mut md = markdown_to_html(m.as_ptr(), dst);
            if md != 0 {
                Some((*dst, md as usize))
            } else {
                None
            }
        }
    }

    pub fn load_markdown(&mut self, markdown: &str) -> bool {
        let html = HtmlBody::_load_markdown(markdown);
        match html {
            Some(x) => {
                self.raw_body = Some(x.0);
                unsafe {
                    self.body = String::from_raw_parts(x.0, x.1, x.1);
                }
                true
            }
            _ => false,
        }
    }

    pub fn as_string(&self) -> &String {
        &self.body
    }
}
impl Drop for HtmlBody {
    fn drop(&mut self) {
        match self.raw_body {
            Some(x) => unsafe {
                markdown_free(x);
            },
            None => {}
        }
    }
}
impl Deref for HtmlBody {
    type Target = String ; // 目标类型
    fn deref<'a>(&'a self) -> &'a String{
        &self.body // 返回String类型的引用
    }
}
