use crate::sys::id;

const UTF8_ENCODING: usize = 4;

pub trait NSString {
    unsafe fn alloc(_: Self) -> id;
    unsafe fn init_str(self, s: &str) -> id;
    unsafe fn utf8_string(self) -> *const u8;
    unsafe fn len(self) -> usize;
}

impl NSString for id {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSString), alloc]
    }

    unsafe fn init_str(self, s: &str) -> id {
        msg_send![self, initWithBytes: s.as_ptr()
                               length: s.len()
                             encoding: UTF8_ENCODING as id]
    }

    unsafe fn utf8_string(self) -> *const u8 {
        msg_send![self, UTF8String]
    }

    unsafe fn len(self) -> usize {
        msg_send![self, lengthOfBytesUsingEncoding: UTF8_ENCODING]
    }
}
