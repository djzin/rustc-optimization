#[no_mangle]
pub fn int_vec_len(x: usize) -> usize {
    let vec = vec![0; x];
    vec.len()
}
