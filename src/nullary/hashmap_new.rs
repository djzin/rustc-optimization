#[no_mangle]
pub fn hashmap_new() {
    std::collections::HashMap::<(), ()>::new();
}
