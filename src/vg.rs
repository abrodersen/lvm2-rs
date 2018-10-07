
use context::Context;
use mapper::DeviceMapperList;

use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;

use ffi;

impl Context {
    pub(crate) fn list_volume_groups(&self) -> 
        DeviceMapperList<ffi::lvm_str_list, *mut ffi::dm_list> 
    {
        trace!("listing vgs, context = {:p}", self.ptr);
        let list = unsafe { ffi::lvm_list_vg_names(self.ptr) };

        trace!("listing vgs, list = {:p}", list);

        if list == ptr::null_mut() {
            let error = self.last_error();
            panic!("vg list error: {}", error);
        }

        unsafe { 
            DeviceMapperList::<ffi::lvm_str_list, *mut ffi::dm_list>::create(list)
        }
    }

    pub(crate) fn open_volume_group(&self, name: &str) -> 
        VolumeGroupHandle 
    {
        trace!("opening volume group {}", name);
        let native_name = CString::new(name)
            .expect("invalid volume group name");
        let mode = CString::new("r")
            .expect("invalid access mode");
        let ptr = unsafe { 
            ffi::lvm_vg_open(
                self.ptr, 
                native_name.as_c_str().as_ptr(),
                mode.as_c_str().as_ptr(),
                0)
        };

        trace!("volume group open, ptr = {:p}", ptr);
        
        VolumeGroupHandle { context: self, ptr: ptr }
    }
}

pub(crate) struct VolumeGroupHandle<'a> {
    pub(crate) context: &'a Context,
    pub(crate) ptr: *mut ffi::volume_group,
}

#[derive(Debug)]
pub struct VolumeGroup {
    pub(crate) name: String,
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
