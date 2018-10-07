
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

    pub fn list_volume_group_names<'a>(&'a self) -> StringList<'a> {
        trace!("listing vg names, context = {:p}", self.ptr);
        let list = unsafe { ffi::lvm_list_vg_names(self.ptr) };
        let handle = list::ListHandle::<CString>::new(self, list);

        StringList {
            inner: handle
        }
    }
}


pub struct StringList<'a> {
    inner: list::ListHandle<'a, CString>,
}

impl<'a, 'b> StringList<'a> {
    pub fn iter(&'b self) -> StringListIterator<'a, 'b> {
        StringListIterator {
            inner: self.inner.iter(),
        }
    }
}

pub struct StringListIterator<'a: 'b, 'b> {
    inner: list::ListHandleIterator<'a, 'b, ffi::lvm_str_list, CString>,
}

impl<'a, 'b: 'a> Iterator for StringListIterator<'a, 'b> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        self.inner.next()
            .map(|c| c.to_str()
                .expect("invalid native string")
                .to_string())
    }
}



impl Drop for Context {
    fn drop(&mut self) {
        unsafe { ffi::lvm_quit(self.ptr) }
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

