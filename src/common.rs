
use ffi;

pub(crate) trait ListHandle {
    fn as_raw(&self) -> *mut ffi::dm_list;
}

impl ListHandle for *mut ffi::dm_list {
    fn as_raw(&self) -> Self {
        self.clone()
    }
}