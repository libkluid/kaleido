use crate::sys::{id, nil};
use crate::sys::appkit::NSScreen;
use crate::sys::foundation::{NSDictionary, NSNumber};

use crate::util::{self, NSArrayExt, NSStringExt};


type CGDirectDisplayID = u32;

#[allow(non_upper_case_globals)]
static NSScreenNumber: &'static str = "NSScreenNumber";

fn device_id(screen: id) -> CGDirectDisplayID {
    let _pool = util::AutoreleasePool::new();
    let key = util::make_nsstring(NSScreenNumber);

    unsafe {
        let description = NSScreen::device_description(screen);
        let number_object = description.object_for_key(key);
        number_object.as_u32() as CGDirectDisplayID
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Screen {
    inner: id,
    device_id: CGDirectDisplayID,
    name: String,
    brightness: Option<f32>,
}

impl Screen {
    pub fn set_brightness(&mut self, value: f32) -> bool {
        crate::external::display_services::set_brightness(self.device_id, value)
    }
}

fn query_screen_inner(screen: id) -> Option<Screen> {
    assert_eq!(screen.is_null(), false, "argument must not be null");

    unsafe {
        let device_id = device_id(screen);
        let localized_name = screen.localized_name();
        let name = localized_name.string();
        let brightness = crate::external::display_services::get_brightness(device_id);

        Some(Screen {
            inner: screen,
            device_id,
            name,
            brightness,
        })
    }
}

pub fn query_main_screen() -> Option<Screen> {
    unsafe {
        let inner = NSScreen::main_screen(nil);
        match inner.is_null() {
            true => None,
            false => query_screen_inner(inner),
        }
    }
}

pub fn query_all_screens() -> Vec<Screen> {
    unsafe {
        let screens = NSScreen::screens(nil);
        screens
            .vector()
            .into_iter()
            .map(query_screen_inner)
            .flatten()
            .collect()
    }
}
