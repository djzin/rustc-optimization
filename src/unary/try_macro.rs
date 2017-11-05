#[no_mangle]
pub fn try_macro(x: Result<i32, i32>) -> Result<i32, i32> {
    Ok(try!(x))
}
