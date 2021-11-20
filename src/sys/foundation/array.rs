use crate::sys::id;

pub trait NSArray {
    unsafe fn object_at_index(self, index: u64) -> id;
    unsafe fn count(self) -> u64;
}

impl NSArray for id {
    unsafe fn object_at_index(self, index: u64) -> id {
        msg_send![self, objectAtIndex: index]
    }

    unsafe fn count(self) -> u64 {
        msg_send![self, count]
    }
}
