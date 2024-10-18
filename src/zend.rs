use core::ptr::{null, null_mut};

#[repr(C)]
pub enum Result {
    Success = 0,
    Failure = -1,
}

#[repr(C)]
pub struct IniEntry {}

#[repr(C)]
pub struct ModuleDep {}

#[repr(C)]
pub struct FunctionEntry {}

#[repr(C)]
pub struct ModuleEntry {
    pub size: libc::c_ushort,
    pub zend_api: libc::c_uint,
    pub zend_debug: libc::c_uchar,
    pub zts: libc::c_uchar,
    pub ini_entry: *const IniEntry,
    pub deps: *const ModuleDep,
    pub name: *const libc::c_char,
    pub functions: *const FunctionEntry,
    pub module_startup_func: Option<unsafe extern "C" fn(libc::c_int, libc::c_int) -> Result>,
    pub module_shutdown_func: Option<unsafe extern "C" fn(libc::c_int, libc::c_int) -> Result>,
    pub request_startup_func: Option<unsafe extern "C" fn(libc::c_int, libc::c_int) -> Result>,
    pub request_shutdown_func: Option<unsafe extern "C" fn(libc::c_int, libc::c_int) -> Result>,
    pub info_func: Option<unsafe extern "C" fn(*mut ModuleEntry) -> Result>,
    pub version: *const libc::c_char,
    pub globals_size: libc::size_t,

    #[cfg(php_zts)]
    pub globals_id_ptr: *mut ts_rsrc_id,
    #[cfg(not(php_zts))]
    pub globals_ptr: *mut libc::c_void,

    pub globals_ctor: Option<extern "C" fn(*mut libc::c_void)>,
    pub globals_dtor: Option<extern "C" fn(*mut libc::c_void)>,
    pub post_deactivate_func: Option<extern "C" fn() -> Result>,
    pub module_started: libc::c_int,
    pub r#type: libc::c_uchar,
    pub handle: *mut libc::c_void,
    pub module_number: libc::c_int,
    pub build_id: *const libc::c_char,
}

impl Default for ModuleEntry {
    fn default() -> ModuleEntry {
        ModuleEntry {
            size: 0,
            zend_api: 0,
            zend_debug: 0,
            zts: 0,
            ini_entry: null(),
            deps: null(),
            name: null(),
            functions: null(),
            module_startup_func: None,
            module_shutdown_func: None,
            request_startup_func: None,
            request_shutdown_func: None,
            info_func: None,
            version: null(),
            globals_size: 0,
            globals_ptr: null_mut(),
            globals_ctor: None,
            globals_dtor: None,
            post_deactivate_func: None,
            module_started: 0,
            r#type: 0,
            handle: null_mut(),
            module_number: 0,
            build_id: null(),
        }
    }
}
