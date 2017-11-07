pub enum Two { First, Second }
use Two::*;

#[no_mangle]
pub fn two_valued_enum(x: Two) -> Two {
    match x {
        First => First,
        Second => Second,
    }
}
