pub mod instance;

pub use instance::Instance;
pub use instance::Controller;
pub use instance::Error;
pub use instance::ControllerBackend;

#[cfg(test)]
mod tests {
    use crate::Instance;

    #[test]
    fn it_works() {
        let instance = Instance::new().unwrap();
        let controllers = instance.controllers();
        for controller in controllers {
            let old_saturation = controller.get_saturation(&instance);
            println!("{} ({}): {}", controller.get_backend(),
                     controller.get_name(), old_saturation);
            controller.set_saturation(&instance, 1.0);
            controller.set_saturation(&instance, old_saturation);
        }
    }
}
