#[no_mangle]
pub fn vec_len(x: usize) -> usize {
    let vec = vec![(); x];
    vec.len()
}
