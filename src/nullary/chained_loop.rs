#[no_mangle]
pub fn chained_loop() {
    (0..1000).chain(0..1000).for_each(|_| ())
}
