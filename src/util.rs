use crate::sys::{id, nil};
use crate::sys::foundation::{NSAutoreleasePool, NSArray, NSString};

pub struct AutoreleasePool {
    inner: id,
}

impl AutoreleasePool {
    pub fn new() -> Self {
        let inner: id = unsafe { NSAutoreleasePool::new(nil) };
        AutoreleasePool { inner }
    }
}

impl Drop for AutoreleasePool {
    fn drop(&mut self) {
        unsafe { NSAutoreleasePool::drain(self.inner) }
    }
}

pub fn make_nsstring(s: &str) -> id {
    unsafe {
        NSString::alloc(nil)
            .init_str(s)
            .autorelease()
    }
}

pub trait NSStringExt: NSString {
    unsafe fn string(self) -> String;
}

impl NSStringExt for id {
    unsafe fn string(self) -> String {
        let ptr = self.utf8_string();
        let slice = std::slice::from_raw_parts(ptr, self.len());
        String::from_utf8_unchecked(Vec::from(slice))
    }
}

pub trait NSArrayExt: NSArray {
    unsafe fn vector(self) -> Vec<id>;
}

impl NSArrayExt for id {
    unsafe fn vector(self) -> Vec<id> {
        let len = self.count();
        (0..len)
            .map(|index| self.object_at_index(index))
            .collect()
    }
}
