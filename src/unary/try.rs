#[no_mangle]
pub fn try(x: Result<i32, i32>) -> Result<i32, i32> {
    Ok(x?)
}
