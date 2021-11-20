use crate::sys::id;

pub trait NSDictionary {
    unsafe fn object_for_key(self, key: id) -> id;
}

impl NSDictionary for id {
    unsafe fn object_for_key(self, key: id) -> id {
        msg_send![self, objectForKey: key]
    }
}
