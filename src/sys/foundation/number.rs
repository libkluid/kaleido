use crate::sys::id;

pub trait NSNumber {
    unsafe fn as_u32(self) -> u32;
}

impl NSNumber for id {
    unsafe fn as_u32(self) -> u32 {
        msg_send![self, unsignedIntegerValue]
    }
}
