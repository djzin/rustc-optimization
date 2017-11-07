pub enum Four { First, Second, Third, Fourth }
use Four::*;

#[no_mangle]
pub fn four_valued_enum(x: Four) -> Four {
    match x {
        First => First,
        Second => Second,
        Third => Third,
        Fourth => Fourth,
    }
}
