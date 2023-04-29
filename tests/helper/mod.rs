pub mod rand;

use math::big::Int;

pub fn is_big_int_normalized(x: &Int) -> bool {
    match x.bits().next_back() {
        None => x.sign() == 0,
        Some(v) => v != 0,
    }
}
