pub enum Three { First, Second, Third }
use Three::*;

#[no_mangle]
pub fn three_valued_enum(x: Three) -> Three {
    match x {
        First => First,
        Second => Second,
        Third => Third,
    }
}
