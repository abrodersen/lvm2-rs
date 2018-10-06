
use ffi;

use std::ptr;
use std::ffi::{CString, CStr};

pub struct Context {
    ctx: ffi::lvm_t,
}

impl Context {
    pub fn new() -> Context {
        Context {
            ctx: unsafe { ffi::lvm_init(ptr::null()) },
        }
    }

    pub fn last_error(&self) -> Error {
        let ptr = unsafe { 
            CStr::from_ptr(ffi::lvm_errmsg(self.ctx)) 
        };
        let msg = ptr.to_str()
            .expect("invalid error message")
            .to_string();
        Error {
            errno: unsafe { ffi::lvm_errno(self.ctx) },
            msg: msg,
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { ffi::lvm_quit(self.ctx) }
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
        assert_ne!(ctx.ctx, ptr::null_mut());
    }
}

