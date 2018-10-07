
use ffi;

pub struct VolumeGroup {
    ptr: *mut ffi::lvm_vg,
}

impl Context {
    pub fn list_volume_groups(&self) -> Vec<VolumeGroup> {
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