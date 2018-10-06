
extern crate lvm2_sys as ffi;
extern crate failure;
#[macro_use] extern crate failure_derive;

mod context;
mod mapper;
mod vg;

pub use vg::{VolumeGroup, list_volume_groups};
