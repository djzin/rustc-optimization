#[no_mangle]
pub fn simple_loop() {
    (0..1000).for_each(|_| ())
}
