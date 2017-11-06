#[no_mangle]
pub fn try_option(x: Option<i32>) -> Option<i32> {
    Some(x?)
}
