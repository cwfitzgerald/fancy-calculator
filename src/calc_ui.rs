/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Backend {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Callbacks {
    pub init: ::std::option::Option<
        unsafe extern "C" fn(data: *mut ::std::os::raw::c_void, internal: *mut Backend),
    >,
    pub updatedTextField: ::std::option::Option<
        unsafe extern "C" fn(data: *mut ::std::os::raw::c_void, text: *mut ::std::os::raw::c_char),
    >,
    pub solve: ::std::option::Option<unsafe extern "C" fn(data: *mut ::std::os::raw::c_void)>,
    pub deleteData: ::std::option::Option<unsafe extern "C" fn(data: *mut ::std::os::raw::c_void)>,
    pub data: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_Callbacks() {
    assert_eq!(
        ::std::mem::size_of::<Callbacks>(),
        40usize,
        concat!("Size of: ", stringify!(Callbacks))
    );
    assert_eq!(
        ::std::mem::align_of::<Callbacks>(),
        8usize,
        concat!("Alignment of ", stringify!(Callbacks))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Callbacks>())).init as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Callbacks),
            "::",
            stringify!(init)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Callbacks>())).updatedTextField as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Callbacks),
            "::",
            stringify!(updatedTextField)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Callbacks>())).solve as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Callbacks),
            "::",
            stringify!(solve)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Callbacks>())).deleteData as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(Callbacks),
            "::",
            stringify!(deleteData)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Callbacks>())).data as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(Callbacks),
            "::",
            stringify!(data)
        )
    );
}
extern "C" {
    pub fn runGui(callbacks: Callbacks) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setAnswer(internal: *mut Backend, value: *mut ::std::os::raw::c_char);
}
