pub mod instance;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let i = crate::instance::Instance::new().unwrap();
        let c = i.controllers();
        for controller in c {
            println!("{}: {}", controller.get_name(), controller.get_saturation(&i.xcon));
            controller.set_saturation(&i.xcon, 1.0);
        }
    }
}
