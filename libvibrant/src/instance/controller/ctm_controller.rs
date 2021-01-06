use crate::instance::xwrapper::RROutput;
use crate::instance::controller::{Controller, SATURATION_MIN, SATURATION_MAX, ControllerBackend};
use std::os::raw::{c_long, c_ulong};
use std::slice::from_raw_parts;
use x11::{xlib, xrandr};
use std::mem::size_of;
use x11::xlib::XA_INTEGER;
use crate::instance::Instance;

pub struct CTMController {
    output: RROutput,
    ctm_prop: xlib::Atom,
    name: String
}

impl CTMController {
    pub fn new(output: RROutput, ctm_prop: xlib::Atom) -> CTMController {
        CTMController{
            name: output.name(),
            output,
            ctm_prop
        }
    }
}

impl Controller for CTMController {
    fn get_saturation(&self, instance: &Instance) -> f64 {
        let xcon = instance.xcon();
        //get the actual color matrix
        let mut ctm: [u64; 9] = [0; 9];
        unsafe {
            let mut actual_type = 0;
            let mut actual_format = 0;
            let mut item_count = 0;
            let mut bytes_after: c_ulong = 0;
            let mut data_ptr: *const c_long = std::ptr::null();
            xrandr::XRRGetOutputProperty(xcon, self.output.id(), self.ctm_prop, 0,
                                         size_of::<c_long>() as c_long * 18, 0, 0, XA_INTEGER,
                                         &mut actual_type as *mut _, &mut actual_format as *mut _,
                                         &mut item_count as *mut _, &mut bytes_after as *mut _,
                                         &mut data_ptr as *mut _ as *mut _);
            if actual_type == XA_INTEGER && item_count == 18u64 {
                let data_ptr = from_raw_parts(data_ptr, 18);
                //see the set_saturation function for why this translation is needed
                for i in (0..18).step_by(2) {
                    ctm[i/2] = (data_ptr[i+1] as u64) << 32 | (data_ptr[i] as u64);
                }
            }
        };

        //translate the matrix into the coeffs
        let mut coeffs: [f64; 9] = [0.0; 9];
        for i in 0..9 {
            //we need to clear the sign bit if we want to convert it into a floating point
            let ctm_num = ctm[i] & !((1 as u64) << 63);
            let mut coeff = (ctm_num as f64)/f64::powi(2.0, 32);
            //recover original sign
            if (ctm[i] & ((1 as u64) << 63)) != 0 {
                coeff *= -1.0;
            }

            coeffs[i] = coeff;
        }

        coeffs[0] - coeffs[1]
    }

    fn set_saturation(&self, instance: &Instance, mut saturation: f64) {
        let xcon = instance.xcon();
        saturation = f64::max(saturation, SATURATION_MIN);
        saturation = f64::min(saturation, SATURATION_MAX);

        let mut ctm_coeffs: [f64; 9] = [0.0; 9];
        let coeff = (1.0 -  saturation) / 3.0;
        for (idx, val) in ctm_coeffs.iter_mut().enumerate() {
            if idx % 4 == 0 {
                *val = coeff + saturation;
            }
            else {
                *val = coeff + 0.0;
            }
        }

        let mut ctm: [u64; 9] = [0; 9];
        //translate the coeffs into a CTM
        for i in 0..9 {
            if ctm_coeffs[i] < 0.0 {
                ctm[i] = (-ctm_coeffs[i] * ((1 as u64) << 32) as f64) as u64;
                ctm[i] |= (1 as u64) << 63;
            }
            else {
                ctm[i] = (ctm_coeffs[i] * ((1 as u64) << 32) as f64) as u64;
            }
        }

        /* The format for CTM is supposed to be a 3x3 matrix of type S31.32, libdrm, and the kernel
         * correctly use uint64_t in their code to represent a S31.32 number. X11/Xrandr however,
         * in all their infinite wisdom decided that their CTM should be of type long, and thus used
         * an array of 18 long values. This is because according to the C standard long is at least
         * 32 bits long. What they did not consider is that long is 64 bits on some systems. This
         * means that the matrix on those systems essentially has the type S63.64. To deal with
         * this we have to translate our matrix from holding u64, to holding whatever the system
         * defines as the size for C's long type.
         *
         * Welcome to hell.
         */
        let mut padded_ctm: [c_long; 18] = [0; 18];
        unsafe {
            let ctm: &[i32] = from_raw_parts(ctm.as_ptr() as *const _, 18);
            for i in 0..18 {
                padded_ctm[i] = ctm[i] as c_long;
            }
        }

        // Now that we have our CTM we can actually set the value
        unsafe {
            xrandr::XRRChangeOutputProperty(xcon, self.output.id(), self.ctm_prop,
                                            XA_INTEGER, 32, xlib::PropModeReplace,
                                            padded_ctm.as_ptr() as *const _, 18);
            xlib::XSync(xcon, 0);
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_backend(&self) -> ControllerBackend {
        ControllerBackend::CTM
    }
}
