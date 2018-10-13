
use ffi;

use list;

use std::ptr;
use std::ffi::{CStr, CString};

pub struct Context {
    ptr: ffi::lvm_t,
}

impl Context {
    pub fn new() -> Context {
        let ptr = unsafe { ffi::lvm_init(ptr::null()) };
        trace!("creating context, ptr = {:p}", ptr);
        Context { ptr: ptr }
    }

    pub fn scan(&self) -> Option<Error> {
        trace!("scanning, ptr = {:p}", self.ptr);
        if unsafe { ffi::lvm_scan(self.ptr) } != 0 {
            Some(self.last_error())
        } else {
            None
        }
    }

    fn last_error(&self) -> Error {
        let ptr = unsafe { 
            CStr::from_ptr(ffi::lvm_errmsg(self.ptr)) 
        };
        let msg = ptr.to_str()
            .expect("invalid error message")
            .to_string();
        Error {
            errno: unsafe { ffi::lvm_errno(self.ptr) },
            msg: msg,
        }
    }

    pub fn list_volume_group_names(&self) -> Vec<String> {
        trace!("listing vg names, context = {:p}", self.ptr);
        let list = unsafe { ffi::lvm_list_vg_names(self.ptr) };
        let handler = list::Handle::new(list);
        list::DeviceMapperIterator::<ffi::lvm_str_list>::new(handler)
            .map(|c: CString| c.to_str()
                .expect("invalid native string")
                .to_string())
            .collect::<Vec<String>>()
    }

    pub fn open_volume_group<'a>(&'a self, name: &str, mode: &Mode) -> VolumeGroup<'a> {
        trace!("opening vg, context = {:p}, name = {}", self.ptr, name);
        let name = CString::new(name).expect("invalid name string");
        let mode = mode.to_c_string();
        let result = unsafe { 
            ffi::lvm_vg_open(self.ptr, name.as_ptr(), mode.as_ptr(), 0) 
        };

        trace!("vg opened, vg = {:p}", result);
        if result != ptr::null_mut() {
            VolumeGroup { ptr: result, ctx: self }
        } else {
            panic!("failed to open vg: {}", self.last_error());
        }
    }
}

pub enum Mode {
    Read,
    ReadWrite,
}

impl Mode {
    fn to_c_string(&self) -> CString {
        let result = match self {
            Mode::Read => CString::new("r"),
            Mode::ReadWrite => CString::new("w"),
        };

        result.expect("invalid mode string")
    }
}


impl Drop for Context {
    fn drop(&mut self) {
        unsafe { ffi::lvm_quit(self.ptr) }
    }
}

pub struct VolumeGroup<'a> {
    ctx: &'a Context,
    ptr: ffi::vg_t,
}

impl<'a> VolumeGroup<'a> {
    pub fn list_logical_volumes<'b>(&'b self) -> Vec<LogicalVolume<'a, 'b>> {
        let list = unsafe { ffi::lvm_vg_list_lvs(self.ptr) };
        if list == ptr::null_mut() {
            panic!("failed to list lvs: {}", self.ctx.last_error());
        }

        let handler = list::Handle::new(list);
        list::DeviceMapperIterator::<ffi::lvm_lv_list>::new(handler)
            .map(|ptr| {
                LogicalVolume { ptr: ptr, _vg: self }
            })
            .collect::<Vec<_>>()
    }
}

impl<'a> Drop for VolumeGroup<'a> {
    fn drop(&mut self) {
        unsafe { ffi::lvm_vg_close(self.ptr) };
    }
}

pub struct LogicalVolume<'a: 'b, 'b> {
    _vg: &'b VolumeGroup<'a>,
    ptr: ffi::lv_t,
}

impl<'a, 'b> LogicalVolume<'a, 'b> {
    pub fn name(&self) -> &'b str {
        let id = unsafe { ffi::lvm_lv_get_name(self.ptr) };
        let wrap = unsafe { CStr::from_ptr::<'b>(id) };
        wrap.to_str().expect("invalid lv name")
    }
}


#[derive(Fail, Debug)]
#[fail(display = "An error occurred with error code {}. ({})", errno, msg)]
pub struct Error {
  errno: i32,
  msg: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn initialize_context() {
        let ctx = Context::new();
        assert_ne!(ctx.ptr, ptr::null_mut());
    }
}

