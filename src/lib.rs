pub mod instance;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let instance = crate::instance::Instance::new().unwrap();
        let controllers = i.controllers();
        for controller in controllers {
            let old_saturation = controller.get_saturation(&instance);
            println!("{} ({}): {}", controller.backend(),
                     controller.get_name(), controller.get_saturation(&instance));
            controller.set_saturation(&instance, 1.0);
            controller.set_saturation(&instance, old_saturation);
        }
    }
}
