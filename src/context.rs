
use ffi;

use std::ptr;
use std::ffi::CStr;

pub(crate) struct Context {
    pub(crate) ptr: ffi::lvm_t,
}

impl Context {
    pub(crate) fn new() -> Context {
        let ptr = unsafe { ffi::lvm_init(ptr::null()) };
        eprintln!("creating context, ptr = {:p}", ptr);
        Context { ptr: ptr }
    }

    pub(crate) fn scan(&self) -> Option<Error> {
        if unsafe { ffi::lvm_scan(self.ptr) } != 0 {
            Some(self.last_error())
        } else {
            None
        }
    }

    pub(crate) fn last_error(&self) -> Error {
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
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { ffi::lvm_quit(self.ptr) }
    }
}

#[derive(Fail, Debug)]
#[fail(display = "An error occurred with error code {}. ({})", errno, msg)]
pub(crate) struct Error {
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

