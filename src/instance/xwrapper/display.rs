use x11::xlib;
use libXNVCtrl_sys as nvctrl;
use crate::instance::error::Error;
use std::ffi::{CStr};
use std::ptr::{null, null_mut};

pub struct Display {
    xcon: *mut xlib::Display,
    has_nvidia: bool
}

impl Display {
    pub fn from_display_name(name: Option<&CStr>) -> Result<Display, Error> {
        // pointer to the name we give to the XOpenDisplay function
        let name_ptr;
        // the name that is returned in error messages
        let name = if let Some(name) = name {
            name_ptr = name.as_ptr();
            String::from(name.to_string_lossy())
        }
        else{
            name_ptr = null();
            String::from("Null")
        };

        let xcon;
        let has_nvidia;

        unsafe {
            xcon = xlib::XOpenDisplay(name_ptr);
            if xcon.is_null() {
                return Err(Error::OpenDisplay(String::from(name)))
            }

            has_nvidia = nvctrl::XNVCTRLQueryExtension(xcon, null_mut(), null_mut()) != 0;
        };


        Ok(Display{
            xcon,
            has_nvidia
        })
    }

    pub fn has_nvidia(&self) -> bool {
        self.has_nvidia
    }

    pub fn xcon(&self) -> *mut xlib::Display {
        self.xcon
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe {
            xlib::XCloseDisplay(self.xcon)
        };
    }
}
