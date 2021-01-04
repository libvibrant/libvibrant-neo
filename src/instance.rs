mod controller;
mod error;
mod xwrapper;

pub use controller::Controller;
pub use crate::instance::error::Error;
use crate::instance::xwrapper::Display;
use std::ffi::CStr;

/// A libvibrant instance. Holds a connection the X server and a list of displays that have
/// available controllers.
pub struct Instance {
    xcon: Display,
    controllers: Vec<Box<dyn Controller>>
}

impl Instance {
    /// Creates a new vibrant instance with a connection to the default X server.
    ///
    /// # Errors
    ///
    /// Returns an error if a connection could not be established to the server.
    pub fn new() -> Result<Instance, Error> {
        let xcon = Display::from_display_name(None)?;
        let controllers = controller::get_controllers(&xcon);
        Ok(Instance {
            xcon,
            controllers
        })
    }

    /// Creates a new vibrant instance with a connection to the specified X server.
    ///
    /// # Errors
    ///
    /// Returns an error if a connection could not be established to the server.
    pub fn from_display_name(name: &CStr) -> Result<Instance, Error> {
        let xcon = Display::from_display_name(Some(name))?;
        let controllers = controller::get_controllers(&xcon);
        Ok(Instance {
            xcon,
            controllers
        })
    }

    /// Returns a list of controllers that correspond to displays that have a controllable backend.
    pub fn controllers(&self) -> &[Box<dyn Controller>] {
        &self.controllers
    }


    /// Returns a pointer to the X display.
    pub fn xcon(&self) -> *mut x11::xlib::Display {
        self.xcon.xcon()
    }
}
