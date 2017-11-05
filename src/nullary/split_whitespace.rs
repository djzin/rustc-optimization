#[no_mangle]
pub fn split_whitespace() {
    "string with spaces".split_whitespace().for_each(|_| ())
}
