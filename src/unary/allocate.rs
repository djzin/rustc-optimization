#[no_mangle]
pub fn allocate(x: usize) -> usize {
    let _ = Vec::<u8>::with_capacity(x);
    x
}
