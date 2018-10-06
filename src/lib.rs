
extern crate lvm2_sys as ffi;
extern crate failure;
#[macro_use] extern crate failure_derive;
#[macro_use] extern crate log;


mod context;
mod mapper;
mod vg;

pub use vg::{VolumeGroup, list_volume_groups};
