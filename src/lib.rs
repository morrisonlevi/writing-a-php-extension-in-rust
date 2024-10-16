mod zend;

#[no_mangle]
pub extern "C" fn get_module() -> *mut zend::ModuleEntry {
    Box::leak(Box::new(zend::ModuleEntry {
        zend_api: 20230831,
        name: b"native\0".as_ptr().cast(),
        build_id: b"API20230831,NTS\0".as_ptr().cast(),
        ..Default::default()
    }))
}

/*
#[no_mangle]
pub extern "C" fn get_module() -> *mut zend::ModuleEntry {
    Box::leak(Box::new(zend::ModuleEntry {
        size: core::mem::size_of::<zend::ModuleEntry>() as libc::c_ushort,
        zend_api: 20230831,
        zend_debug: 0,
        name: b"native\0".as_ptr().cast(),
        build_id: b"API20230831,NTS\0".as_ptr().cast(),
        ..Default::default()
    }))
}
*/
