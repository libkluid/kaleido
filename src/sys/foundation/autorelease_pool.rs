use crate::sys::id;

pub trait NSAutoreleasePool {
    unsafe fn new(_: Self) -> id;
    unsafe fn autorelease(self) -> id;
    unsafe fn drain(self);
}

impl NSAutoreleasePool for id {
    unsafe fn new(_: Self) -> id {
        msg_send![class!(NSAutoreleasePool), new]
    }

    unsafe fn autorelease(self) -> id {
        msg_send![self, autorelease]
    }

    unsafe fn drain(self) {
        msg_send![self, drain]
    }
}
