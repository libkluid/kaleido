#[allow(non_camel_case_types)]
pub type id = *mut objc::runtime::Object;

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
pub const nil: id = 0x00 as id;

#[link(name = "AppKit", kind = "framework")]
pub mod appkit;

#[link(name = "Foundation", kind = "framework")]
pub mod foundation;
