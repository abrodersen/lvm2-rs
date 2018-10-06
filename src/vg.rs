
use context::Context;
use mapper::{Element, DeviceMapperList};

use std::ffi::CStr;
use std::mem;
use std::ptr;

use ffi;

#[derive(Debug)]
pub struct VolumeGroup {
    name: String,
}

impl Element for ffi::lvm_str_list_t {
    fn next_offset() -> usize {
        let addr: Self = unsafe { mem::uninitialized() };
        let base = &addr as *const _ as usize;
        let next = &addr.list.n as *const _ as usize;
        next - base
    }
}

impl Context {
    pub(crate) fn list_volume_groups(&self) -> 
        DeviceMapperList<ffi::lvm_str_list> 
    {
        trace!("listing vgs, context = {:p}", self.ptr);
        let list = unsafe { ffi::lvm_list_vg_names(self.ptr) };

        trace!("listing vgs, list = {:p}", list);

        if list == ptr::null_mut() {
            let error = self.last_error();
            panic!("vg list error: {}", error);
        }

        unsafe { DeviceMapperList::<ffi::lvm_str_list>::create(list) }
    }
}

pub fn list_volume_groups() -> Vec<VolumeGroup> {
    let context = Context::new();

    if let Some(err) = context.scan() {
        panic!("scanning error: {}", err)
    }

    context.list_volume_groups().iter().map(|e| {
        let ptr = unsafe {
            CStr::from_ptr((*e).str)
        };

        let name = ptr.to_str()
            .expect("invalid VG name")
            .to_string();

        VolumeGroup {
            name: name,
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn list_volue_groups_test() {
        for group in list_volume_groups() {
            println!("volume group: {}", group.name);
        }
    }
}
