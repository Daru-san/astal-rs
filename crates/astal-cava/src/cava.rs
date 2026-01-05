use glib::object::IsA;
use glib::object::ObjectType;

use crate::Cava;
use crate::ffi;

pub trait CavaExtManual: IsA<Cava> + ObjectType<GlibType = ffi::AstalCavaCava> {
    fn values(&self) -> Vec<f64> {
        unsafe {
            let garray = ffi::astal_cava_cava_get_values(self.to_glib_none().0);
            let val = *garray;
            let slice: &[f64] =
                std::slice::from_raw_parts(val.data as *const f64, val.len as usize);
            slice.to_vec()
        }
    }
}

impl<T: IsA<Cava> + ObjectType<GlibType = ffi::AstalCavaCava>> CavaExtManual for T {}
