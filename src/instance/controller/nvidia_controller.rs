use std::os::raw::c_int;
use crate::instance::xwrapper::RROutput;
use crate::instance::controller::{Controller, SATURATION_MIN, SATURATION_MAX, ControllerBackend};
use libXNVCtrl_sys as nvctrl;
use crate::instance::Instance;

pub struct NvidiaController {
    _output: RROutput,
    nvidia_id: c_int,
    name: String
}

impl NvidiaController {
    pub fn new(output: RROutput, nvidia_id: c_int) -> NvidiaController {
        NvidiaController {
            name: output.name(),
            _output: output,
            nvidia_id
        }
    }
}

impl Controller for NvidiaController {
    fn get_saturation(&self, instance: &Instance) -> f64 {
        let xcon = instance.xcon();
        let mut nv_saturation = 0;
        unsafe {
            nvctrl::XNVCTRLQueryTargetAttribute(xcon,
                                                nvctrl::NV_CTRL_TARGET_TYPE_DISPLAY, self.nvidia_id,
                                                0, nvctrl::NV_CTRL_DIGITAL_VIBRANCE,
                                                &mut nv_saturation as *mut _);
        };

        if nv_saturation < 0 {
            (nv_saturation+1024) as f64/1024.0
        }
        else{
            (nv_saturation*3+1023) as f64/1023.0
        }
    }

    fn set_saturation(&self, instance: &Instance, mut saturation: f64) {
        let xcon = instance.xcon();
        let nv_saturation;

        saturation = f64::max(saturation, SATURATION_MIN);
        saturation = f64::min(saturation, SATURATION_MAX);

        //is saturation roughly in [0.0, 1.0]
        if saturation >= 0.0 && saturation <= 1.0 + f64::EPSILON {
            nv_saturation = (saturation * 1024.0 - 1024.0) as i32;
        } else {
            nv_saturation = ((saturation * 1023.0 - 1023.0) / 3.0) as i32;
        }

        unsafe {
            nvctrl::XNVCTRLSetTargetAttribute(xcon, nvctrl::NV_CTRL_TARGET_TYPE_DISPLAY,
                                              self.nvidia_id, 0, nvctrl::NV_CTRL_DIGITAL_VIBRANCE,
                                              nv_saturation);
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_backend(&self) -> ControllerBackend {
        ControllerBackend::XNVCtrl
    }
}
