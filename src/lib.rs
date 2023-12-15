#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub use core_foundation_sys;
pub use io_kit_sys;

pub mod base;
pub mod callbacks;
pub mod disk;
pub mod dissenter;
pub mod session;

pub(crate) mod prelude {
    pub use crate::base::*;
    pub use crate::callbacks::*;
    pub use crate::disk::*;
    pub use crate::dissenter::*;
    pub use crate::session::*;
    pub use core_foundation_sys::{
        array::CFArrayRef, base::*, dictionary::CFDictionaryRef, runloop::CFRunLoopRef,
        string::CFStringRef, url::CFURLRef,
    };
    pub use io_kit_sys::{base::dispatch_queue_t, types::io_service_t, IOObjectRelease};
    pub use std::os::raw::*;
}

// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
