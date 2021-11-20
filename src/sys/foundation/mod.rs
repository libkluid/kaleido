#[path = "array.rs"]
mod nsarray;
pub use nsarray::NSArray;

#[path = "dictionary.rs"]
mod nsdictionary;
pub use nsdictionary::NSDictionary;

#[path = "string.rs"]
mod nsstring;
pub use nsstring::NSString;

#[path = "autorelease_pool.rs"]
mod nsautorelease_pool;
pub use nsautorelease_pool::NSAutoreleasePool;

#[path = "number.rs"]
mod nsnumber;
pub use nsnumber::NSNumber;
