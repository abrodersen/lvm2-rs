
extern crate lvm2_sys as ffi;
extern crate failure;
#[macro_use] extern crate failure_derive;

mod context;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
