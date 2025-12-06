#![cfg_attr(docsrs, feature(doc_cfg))]

macro_rules! assert_initialized_main_thread {
    () => {};
}

macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(unsafe_op_in_unsafe_fn)]
mod auto;
pub mod prelude;
pub use auto::*;

use astal_river_sys as ffi;
