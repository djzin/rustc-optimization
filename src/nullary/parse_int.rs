#[no_mangle]
pub fn parse_int() {
    let _: i32 = "12345".parse().unwrap();
}
