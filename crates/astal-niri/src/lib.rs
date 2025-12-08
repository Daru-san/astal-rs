#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(deprecated)]
#![allow(unused_imports)]

macro_rules! assert_initialized_main_thread {
    () => {};
}

macro_rules! skip_assert_initialized {
    () => {};
}

macro_rules! cast_optional {
    ($op:expr) => {{
        if let Some(val) = $op {
            val as *mut _
        } else {
            std::ptr::null_mut()
        }
    }};
}

use astal_niri_sys as ffi;
pub use auto::*;

mod auto;
pub mod prelude;

mod physical_size;
pub use physical_size::PhysicalSize;

pub mod functions {
    pub use super::auto::functions::*;
}
