
use ffi;

use common::ListHandle;

use std::marker::PhantomData;
use std::mem;

pub trait Element {
    fn next_offset() -> usize;
}

impl Element for ffi::lvm_str_list_t {
    fn next_offset() -> usize {
        let addr: Self = unsafe { mem::uninitialized() };
        let base = &addr as *const _ as usize;
        let next = &addr.list.n as *const _ as usize;
        next - base
    }
}

pub(crate) struct DeviceMapperList<T: Element, H: ListHandle> {
  handle: H,
  _marker: PhantomData<T>,
}

impl<T: Element, H: ListHandle> DeviceMapperList<T, H> {
    pub(crate) unsafe fn create(handle: H) -> DeviceMapperList<T, H> {
        DeviceMapperList {
            handle: handle,
            _marker: PhantomData::<T>
        }
    }

    pub(crate) fn iter<'a>(&'a self) -> DeviceMapperListIterator<'a, T, H> {
        let next = unsafe { *self.handle.as_raw() };
        DeviceMapperListIterator::<'a, T, H> {
            list: self,
            pos: next.n,
        }
    }
}

pub(crate) struct DeviceMapperListIterator<'a, T: 'a + Element, H: 'a + ListHandle> {
    list: &'a DeviceMapperList<T, H>,
    pos: *const ffi::dm_list,
}

impl<'a, T: Element, H: ListHandle> Iterator for DeviceMapperListIterator<'a, T, H> {
    type Item = *const T;

    fn next(&mut self) -> Option<Self::Item> {
        trace!("pos = {:p}, head = {:p}", self.pos, self.list.handle.as_raw());
        if self.pos != self.list.handle.as_raw() {
            let item = get_list_item::<T>(self.pos);
            let next = unsafe { *self.pos };
            self.pos = next.n;
            Some(item)
        } else {
            None
        }
    }
}

fn get_list_item<T: Element>(item: *const ffi::dm_list) -> *const T {
    let next = T::next_offset();
    let item_addr = item as *const _ as usize;
    (item_addr - next) as *const T
}