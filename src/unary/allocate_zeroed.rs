#[no_mangle]
pub fn allocate_zeroed(x: usize) -> usize {
    let _ = vec![0u8; x];
    x
}
