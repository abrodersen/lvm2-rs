
use common::ListHandle;
use context::Context;
use mapper::{Element, DeviceMapperList};
use vg::{VolumeGroup, VolumeGroupHandle};

use std::mem;
use std::ptr;
use std::ffi::CStr;

use ffi;

pub(crate) struct LogicalVolumeListHandle {
    ptr: *mut ffi::dm_list, 
}

impl ListHandle for LogicalVolumeListHandle {
    fn as_raw(&self) -> *mut ffi::dm_list {
        self.ptr
    }
}

impl Drop for LogicalVolumeListHandle {
    fn drop(&mut self) {
        unsafe { ffi::lvm_list_pvs_free(self.ptr); }
    }
}

impl Element for ffi::lvm_lv_list {
    fn next_offset() -> usize {
        let addr: Self = unsafe { mem::uninitialized() };
        let base = &addr as *const _ as usize;
        let next = &addr.list.n as *const _ as usize;
        next - base
    }
}

impl<'a> VolumeGroupHandle<'a> {
    pub(crate) fn list_logical_volumes(&self) -> DeviceMapperList<ffi::lvm_lv_list, LogicalVolumeListHandle> {
        trace!("listing lvs, vg = {:p}", self.ptr);
        let list = unsafe { ffi::lvm_vg_list_lvs(self.ptr) };

        trace!("listing lvs, list = {:p}", list);

        if list == ptr::null_mut() {
            let error = self.context.last_error();
            panic!("lv list error: {}", error);
        }

        let handle = LogicalVolumeListHandle { ptr: list };

        unsafe { DeviceMapperList::<ffi::lvm_lv_list, LogicalVolumeListHandle>::create(handle) }
    }
}

#[derive(Debug)]
pub struct LogicalVolume {
    group: String,
    name: String,
}

pub fn list_logical_volumes(vg: &VolumeGroup) -> Vec<LogicalVolume> {
    let context = Context::new();
    let handle = context.open_volume_group(vg.name.as_str());

    handle.list_logical_volumes().iter().map(|e| {
        trace!("lv list element, ptr = {:?}", e);
        
        let lv_handle = unsafe { *e }.lv;

        trace!("lv, ptr = {:p}", lv_handle);

        let name_ptr = unsafe {
            CStr::from_ptr(ffi::lvm_lv_get_name(lv_handle))
        };

        trace!("lv name, ptr = {:p}", name_ptr);

        let name = name_ptr.to_str()
            .expect("invalid LV name")
            .to_string();

        trace!("lv name = {:?}", name);

        LogicalVolume {
            group: vg.name.to_string(),
            name: name,
        }
    }).collect()
}