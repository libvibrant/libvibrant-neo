use x11::{xrandr, xlib};
use super::display::Display;
use std::slice::from_raw_parts;
use std::ffi::CStr;

pub struct RROutput {
    output: xrandr::RROutput,
    info: *mut xrandr::XRROutputInfo
}

impl RROutput {
    pub fn from_display(display: &Display) -> Vec<RROutput> {
        let root;
        let screen_resources    ;
        let mut outputs;
        let outputs_slice;
        unsafe {
            root = xlib::XDefaultRootWindow(display.xcon());
            screen_resources = xrandr::XRRGetScreenResources(display.xcon(), root);
            outputs = Vec::with_capacity((*screen_resources).noutput as usize);
            outputs_slice = from_raw_parts((*screen_resources).outputs,
                                           (*screen_resources).noutput as usize);
        };


        for output in outputs_slice {
            let output = *output;
            let info;
            unsafe {
                info = xrandr::XRRGetOutputInfo(display.xcon(), screen_resources, output);
                if (*info).connection == xrandr::RR_Connected as u16 {
                    outputs.push(RROutput{
                        output,
                        info
                    })
                }
            }
        }

        outputs
    }

    pub fn id(&self) -> xrandr::RROutput {
        self.output
    }

    pub fn name(&self) -> String {
        unsafe {
            let c_str =
                CStr::from_bytes_with_nul_unchecked(from_raw_parts((*self.info).name as *const _,
                                                                   (*self.info).nameLen as usize + 1));
            c_str.to_string_lossy().to_string()
        }
    }
}

impl Drop for RROutput {
    fn drop(&mut self) {
        unsafe {
            xrandr::XRRFreeOutputInfo(self.info);
        }
    }
}
