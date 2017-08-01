extern crate directwrite;

#[test]
fn create_factory() {
    directwrite::Factory::new().unwrap();
}
