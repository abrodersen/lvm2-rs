
extern crate lvm2_sys as ffi;
extern crate failure;
#[macro_use] extern crate failure_derive;
#[macro_use] extern crate log;

mod list;
mod common;
mod context;
mod mapper;

// pub use vg::{VolumeGroup, list_volume_groups};
// pub use lv::{LogicalVolume, list_logical_volumes};

pub use context::Context;
