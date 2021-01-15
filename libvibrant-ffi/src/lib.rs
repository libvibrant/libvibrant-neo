use libvibrant::Instance;
use libvibrant::ControllerBackend;
use std::ptr::null_mut;
use std::os::raw::{c_uchar, c_char};
use std::ffi::CStr;

#[repr(C)]
pub enum Error {
    Ok,
    OpenDisplay,
    NullName,
    BadName,
    OutOfRange
}

#[repr(C)]
pub enum Backend {
    XNVCtrl,
    CTM
}

#[no_mangle]
pub extern "C" fn vibrant_instance_new(mut _ret: *mut *const Instance) -> Error {
    let instance = match Instance::new() {
        Ok(i) => i,
        Err(_) => {
            _ret = null_mut();
            return Error::OpenDisplay
        }
    };

    unsafe{
        *_ret = Box::into_raw(Box::new(instance));
    }
    Error::Ok
}

#[no_mangle]
pub extern "C" fn vibrant_instance_from_display_name(name: *const c_char,
                                                        mut _ret: *mut *const Instance) -> Error {
    if name.is_null() {
        _ret = null_mut();
        return Error::NullName;
    }

    let name = unsafe { CStr::from_ptr(name) };
    let instance = match Instance::from_display_name(name) {
        Ok(i) => i,
        Err(_) => {
            _ret = null_mut();
            return Error::OpenDisplay
        }
    };

    unsafe {
        *_ret = Box::into_raw(Box::new(instance));
    }
    Error::Ok
}

#[no_mangle]
pub extern "C" fn vibrant_instance_free(instance: *mut Instance) {
    if instance.is_null() {
        return;
    }

    unsafe {
        Box::from_raw(instance);
    }
}

//its kind of messy to let C directly have a pointer and iterate over the controllers so we have to
//facilitate that

#[no_mangle]
pub extern "C" fn vibrant_instance_controllers_size(instance: *mut Instance) -> usize {
    assert!(!instance.is_null());

    let instance = unsafe { instance.as_ref().unwrap() };
    instance.controllers().len()
}

#[no_mangle]
pub extern "C" fn vibrant_instance_get_controller_saturation(instance: *mut Instance,
                                                                idx: usize,
                                                                saturation: *mut f64) -> Error {
    assert!(!instance.is_null());
    assert!(!saturation.is_null());

    let instance = unsafe { instance.as_ref().unwrap() };
    let controllers = instance.controllers();

    if idx < controllers.len() {
        unsafe {
            *saturation = controllers.get(idx).unwrap().get_saturation(instance);
        }
        Error::Ok
    }
    else {
        Error::OutOfRange
    }
}

#[no_mangle]
pub extern "C" fn vibrant_instance_set_controller_saturation(instance: *mut Instance,
                                                                idx: usize,
                                                                saturation: f64) -> Error {
    assert!(!instance.is_null());

    let instance = unsafe { instance.as_ref().unwrap() };
    let controllers = instance.controllers();

    if idx < controllers.len() {
        controllers.get(idx).unwrap().set_saturation(instance, saturation);
        Error::Ok
    }
    else {
        Error::OutOfRange
    }
}

#[no_mangle]
pub extern "C" fn vibrant_instance_get_controller_name(instance: *mut Instance,
                                                          idx: usize, str: *mut *const c_uchar,
                                                          len: *mut usize) -> Error {
    assert!(!instance.is_null());
    assert!(!str.is_null());
    assert!(!len.is_null());

    let instance = unsafe {  instance.as_ref().unwrap() };
    let controllers = instance.controllers();

    if idx < controllers.len() {
        let name = controllers.get(idx).unwrap().get_name();
        unsafe {
            *str = name.as_ptr();
            *len = name.len();
        }
        Error::Ok
    }
    else {
        Error::OutOfRange
    }
}

#[no_mangle]
pub extern "C" fn vibrant_instance_get_controller_backend(instance: *mut Instance,
                                                             idx: usize,
                                                             backend: *mut Backend) -> Error {
    assert!(!instance.is_null());
    assert!(!backend.is_null());

    let instance = unsafe { instance.as_ref().unwrap() };
    let controllers = instance.controllers();

    if idx < controllers.len() {
        unsafe {
            *backend = match controllers.get(idx).unwrap().get_backend() {
                ControllerBackend::XNVCtrl => Backend::XNVCtrl,
                ControllerBackend::CTM => Backend::CTM
            };
        };
        Error::Ok
    }
    else {
        Error::OutOfRange
    }
}
