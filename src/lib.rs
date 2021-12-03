
#[allow(dead_code, non_camel_case_types)]
mod bindings;
use std::{os::raw::c_char, ffi::CStr};

use bindings::*;

const error_seen: bool = true;





fn do_create() {
    let hdl = unsafe { libzfs_init() };


}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
