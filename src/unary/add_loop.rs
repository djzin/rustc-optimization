#[no_mangle]
pub fn add_loop(x: usize) -> usize {
    let mut ret = 0;
    for _ in 0..x {
        ret += 1;
    }
    ret
}
