use crate::sys::id;

pub trait NSScreen {
    unsafe fn main_screen(_: Self) -> id;
    unsafe fn screens(_: Self) -> id;
    unsafe fn device_description(self) -> id;
    unsafe fn localized_name(self) -> id;
}

impl NSScreen for id {
    unsafe fn main_screen(_: Self) -> id {
        msg_send![class!(NSScreen), mainScreen]
    }

    unsafe fn screens(_: Self) -> id {
        msg_send![class!(NSScreen), screens]
    }

     unsafe fn device_description(self) -> id {
         msg_send![self, deviceDescription]
     }

     unsafe fn localized_name(self) -> id {
         msg_send![self, localizedName]
     }
}
