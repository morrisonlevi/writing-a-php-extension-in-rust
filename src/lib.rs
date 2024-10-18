mod zend;

#[no_mangle]
pub extern "C" fn get_module() -> *mut zend::ModuleEntry {
    Box::leak(Box::new(zend::ModuleEntry {
        size: core::mem::size_of::<zend::ModuleEntry>() as libc::c_ushort,
        zend_api: 20230831,
        name: b"native\0".as_ptr().cast(),
        build_id: b"API20230831,NTS\0".as_ptr().cast(),
        zend_debug: cfg!(php_debug) as u8,
        zts: cfg!(php_zts) as u8,
        ..Default::default()
    }))
}
