#[no_mangle]
pub fn while_loop(mut x: usize) -> usize {
    let mut ret = 0;
    while x > 0 {
        x -= 1;
        ret += 1;
    }
    ret
}
