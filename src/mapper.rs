
use ffi;

use std::marker::PhantomData;

pub trait Element {
    fn next_offset() -> usize;
}

pub(crate) struct DeviceMapperList<T: Element> {
  head: *const ffi::dm_list,
  _marker: PhantomData<T>,
}

impl<T: Element> DeviceMapperList<T> {
    pub(crate) unsafe fn create(ptr: *const ffi::dm_list) -> DeviceMapperList<T> {
        DeviceMapperList {
            head: ptr,
            _marker: PhantomData::<T>
        }
    }

    pub(crate) fn iter<'a>(&'a self) -> DeviceMapperListIterator<'a, T> {
        let next = unsafe { *self.head };
        DeviceMapperListIterator::<'a, T> {
            list: self,
            pos: next.n,
        }
    }
}

pub(crate) struct DeviceMapperListIterator<'a, T: 'a + Element> {
    list: &'a DeviceMapperList<T>,
    pos: *const ffi::dm_list,
}

impl<'a, T: Element> Iterator for DeviceMapperListIterator<'a, T> {
    type Item = *const T;

    fn next(&mut self) -> Option<Self::Item> {
        trace!("pos = {:p}, head = {:p}", self.pos, self.list.head);
        if self.pos != self.list.head {
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