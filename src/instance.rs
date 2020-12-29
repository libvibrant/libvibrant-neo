pub mod controller;
pub mod error;
mod xwrapper;

use controller::Controller;
use crate::instance::error::Error;
use crate::instance::xwrapper::Display;
use std::ffi::CStr;

struct Instance {
    xcon: Display,
    controllers: Vec<Box<dyn Controller>>
}

impl Instance {
    pub fn new() -> Result<Instance, Error> {
        Self::from_display_name(None)
    }

    pub fn from_display_name(name: Option<&CStr>) -> Result<Instance, Error> {
        let xcon = Display::from_display_name(name)?;
        let controllers = controller::get_controllers(&xcon);
        Ok(Instance {
            xcon,
            controllers
        })
    }
}
