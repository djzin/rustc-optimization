#[no_mangle]
pub fn from_utf8() {
    std::str::from_utf8(b"this is ascii").unwrap();
}
