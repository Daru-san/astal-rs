use crate::ffi;
use glib::translate::{FromGlib, FromGlibPtrNone};
use glib::types::StaticType;
use glib::value::ToValue;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct PhysicalSize {
    pub x: u32,
    pub y: u32,
}

impl FromGlibPtrNone<*mut ffi::AstalNiriPhysicalSize> for PhysicalSize {
    unsafe fn from_glib_none(ptr: *mut ffi::AstalNiriPhysicalSize) -> Self {
        let data = ptr.read();
        Self {
            x: data.x,
            y: data.y,
        }
    }
}

impl StaticType for PhysicalSize {
    fn static_type() -> glib::Type {
        unsafe {
            glib::Type::from_glib(glib::ffi::GType::from(
                ffi::astal_niri_physical_size_get_type(),
            ))
        }
    }
}
