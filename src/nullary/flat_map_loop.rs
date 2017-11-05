#[no_mangle]
pub fn flat_map_loop() {
    (0..1000).flat_map(|n| 0..n).for_each(|_| ())
}
