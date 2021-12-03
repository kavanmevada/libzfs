/* automatically generated by rust-bindgen 0.59.2 */

#[repr(C)]
pub struct nvlist {
    pub nvl_version: i32,
    pub nvl_nvflag: u32,
    pub nvl_priv: u64,
    pub nvl_flag: u32,
    pub nvl_pad: i32,
}
#[test]
fn bindgen_test_layout_nvlist() {
    assert_eq!(
        ::std::mem::size_of::<nvlist>(),
        24usize,
        concat!("Size of: ", stringify!(nvlist))
    );
    assert_eq!(
        ::std::mem::align_of::<nvlist>(),
        8usize,
        concat!("Alignment of ", stringify!(nvlist))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nvlist>())).nvl_version as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(nvlist),
            "::",
            stringify!(nvl_version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nvlist>())).nvl_nvflag as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(nvlist),
            "::",
            stringify!(nvl_nvflag)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nvlist>())).nvl_priv as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(nvlist),
            "::",
            stringify!(nvl_priv)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nvlist>())).nvl_flag as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(nvlist),
            "::",
            stringify!(nvl_flag)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<nvlist>())).nvl_pad as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(nvlist),
            "::",
            stringify!(nvl_pad)
        )
    );
}
impl Default for nvlist {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type nvlist_t = nvlist;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct libzfs_handle {
    _unused: [u8; 0],
}
pub type libzfs_handle_t = libzfs_handle;
extern "C" {
    pub fn libzfs_init() -> *mut libzfs_handle_t;
}
extern "C" {
    pub fn zpool_create(
        arg1: *mut libzfs_handle_t,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut nvlist_t,
        arg4: *mut nvlist_t,
        arg5: *mut nvlist_t,
    ) -> ::std::os::raw::c_int;
}