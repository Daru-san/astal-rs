#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(unused_imports)]
#![allow(deprecated)]

macro_rules! assert_initialized_main_thread {
    () => {};
}

macro_rules! skip_assert_initialized {
    () => {};
}

mod auto;
pub mod prelude;
pub use auto::*;

use astal_river_sys as ffi;

pub mod functions {
    pub use super::auto::functions::*;
}
