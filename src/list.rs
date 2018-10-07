
use context::Context;

use std::ffi::{CStr, CString};
use std::marker::PhantomData;

use ffi;

pub(crate) trait Node {
    type Item;

    fn next(&self) -> *mut ffi::dm_list;
    fn into(*mut ffi::dm_list) -> Self;
    fn item(&self) -> Self::Item;
}

impl Node for ffi::lvm_str_list {
    type Item = CString;

    fn next(&self) -> *mut ffi::dm_list {
        self.list.n
    }

    fn into(ptr: *mut ffi::dm_list) -> Self {
        unsafe { *(ptr as *mut Self) }
    }

    fn item(&self) -> CString {
        unsafe { CStr::from_ptr(self.str).into() }
    }
}

pub(crate) trait Element {
    fn element(*const ffi::dm_list) -> *const Self;
}

pub(crate) struct ListHandle<'a, T> {
    ctx: &'a Context,
    ptr: *mut ffi::dm_list,
    _data: PhantomData<T>,
}

impl<'a, T> ListHandle<'a, T> {
    pub(crate) fn new(ctx: &'a Context, ptr: *mut ffi::dm_list) -> ListHandle<'a, T> {
        ListHandle {
            ctx: ctx,
            ptr: ptr,
            _data: PhantomData::<T>,
        }
    }

    pub(crate) fn iter<'b, N>(&'b self) -> ListHandleIterator<'a, 'b, N, T> 
        where N: Node<Item=T>
    {
        ListHandleIterator {
            handle: self,
            pos: None,
        }
    }
}

pub(crate) struct ListHandleIterator<'a: 'b, 'b, N, T: 'a> 
    where N: Node<Item=T>
{
    handle: &'b ListHandle<'a, T>,
    pos: Option<N>,
}

impl<'a, 'b: 'a, N, T: 'b> Iterator for ListHandleIterator<'a, 'b, N, T> 
    where N: Node<Item=T>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.pos {
            Some(ref p) => p.next(),
            None => unsafe { *self.handle.ptr }.n,
        };

        if next != self.handle.ptr {
            let node = N::into(next);
            let item = node.item();
            self.pos = Some(node);
            Some(item)
        } else {
            None
        }
    }
}
