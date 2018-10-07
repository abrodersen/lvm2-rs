
extern crate lvm2_sys as ffi;
extern crate failure;
#[macro_use] extern crate failure_derive;
#[macro_use] extern crate log;

mod common;
mod context;
mod mapper;
mod vg;
mod lv;

pub use vg::{VolumeGroup, list_volume_groups};
pub use lv::{LogicalVolume, list_logical_volumes};
