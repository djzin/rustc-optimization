#[derive(Debug)]
pub enum Void {}

#[no_mangle]
pub fn result_void(x: Result<i32, Void>) -> i32 {
    x.unwrap()
}
