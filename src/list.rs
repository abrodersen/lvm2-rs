
use context::Context;

use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::mem;

use ffi;

#[derive(PartialEq, Eq, Clone)]
pub(crate) struct Handle<'a> {
    ptr: *mut ffi::dm_list,
    _data: PhantomData<&'a ffi::dm_list>,
}

impl<'a> Handle<'a> {
    pub(crate) fn new(ptr: *mut ffi::dm_list) -> Handle<'a> {
        Handle { 
            ptr: ptr,
            _data: PhantomData::<&'a ffi::dm_list>,
        }
    }

    fn follow<'b>(&'b self) -> Handle<'b> {
        let ptr = unsafe { *self.ptr }.n;
        Handle::new(ptr)
    }
}

pub(crate) trait Node {
    type Item;

    fn next<'a>(&'a self) -> Handle<'a>;
    fn from_handle<'a>(h: Handle<'a>) -> Self;
    fn item(&self) -> Self::Item;
}

impl Node for ffi::lvm_str_list {
    type Item = CString;

    fn next<'a>(&'a self) -> Handle<'a> {
        Handle::new(self.list.n)
    }

    fn from_handle<'a>(h: Handle<'a>) -> Self {
        let addr: Self = unsafe { mem::uninitialized() };
        let base = &addr as *const _ as usize;
        let next = &addr.list.n as *const _ as usize;
        let offset = next - base;

        let ptr = h.ptr as *const _ as usize;
        let self_ptr = (ptr - offset) as *const Self;
        unsafe { *self_ptr }
    }

    fn item(&self) -> CString {
        unsafe { CStr::from_ptr(self.str).into() }
    }
}

pub(crate) struct DeviceMapperIterator<'a, N: Node> {
    base: Handle<'a>,
    pos: *mut ffi::dm_list,
    _data: PhantomData<N>,
}

impl<'a, N: Node> DeviceMapperIterator<'a, N> {
    pub(crate) fn new(h: Handle<'a>) -> DeviceMapperIterator<'a, N> {
        DeviceMapperIterator {
            base: h.clone(),
            pos: h.follow().ptr,
            _data: PhantomData::<N>,
        }
    }
}

impl<'a, N, T> Iterator for DeviceMapperIterator<'a, N> 
    where N: Node<Item=T>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let tmp = Handle::new(self.pos);
        if tmp != self.base {
            let node = N::from_handle(tmp.clone());
            let item = node.item();

            let next = tmp.follow();
            self.pos = next.ptr;
            Some(item)
        } else {
            None
        }
    }
}
